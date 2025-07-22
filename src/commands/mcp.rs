use anyhow::Result;
use rmcp::{
    RoleServer, ServerHandler, ServiceExt,
    model::{ErrorData as McpError, *},
    service::RequestContext,
};
use tokio::io::{stdin, stdout};

#[derive(Clone)]
pub struct PmxMcpServer {
    storage: crate::storage::Storage,
}

impl PmxMcpServer {
    pub fn new(storage: crate::storage::Storage) -> Self {
        Self { storage }
    }

    fn is_prompt_enabled(&self, prompt_name: &str) -> bool {
        match &self.storage.config.mcp.disable_prompts {
            crate::storage::DisableOption::Bool(true) => false,
            crate::storage::DisableOption::Bool(false) => true,
            crate::storage::DisableOption::List(disabled_list) => {
                !disabled_list.contains(&prompt_name.to_string())
            }
        }
    }
}

impl ServerHandler for PmxMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_prompts().build(),
            server_info: Implementation {
                name: "pmx-mcp-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("This server provides system prompts managed by pmx.".to_string()),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        let profiles = self
            .storage
            .list_repos()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let mut prompts = Vec::new();
        for profile in profiles {
            if self.is_prompt_enabled(&profile) {
                prompts.push(Prompt::new(
                    &profile,
                    Some(&format!("System prompt: {}", profile)),
                    None,
                ));
            }
        }

        Ok(ListPromptsResult {
            next_cursor: None,
            prompts,
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments: _ }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        if !self.is_prompt_enabled(&name) {
            return Err(McpError::invalid_params("Prompt is disabled", None));
        }

        let content = self
            .storage
            .get_content(&name)
            .map_err(|e| McpError::invalid_params(format!("Prompt not found: {}", e), None))?;

        Ok(GetPromptResult {
            description: None,
            messages: vec![PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(content),
            }],
        })
    }
}

pub fn run_mcp_server(storage: crate::storage::Storage) -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let service = PmxMcpServer::new(storage);
            let server = service.serve((stdin(), stdout())).await?;
            server.waiting().await?;
            Ok(())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_is_prompt_enabled() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_storage");
        crate::storage::Storage::initialize(path.clone()).unwrap();

        let config = crate::storage::Config {
            agents: crate::storage::Agents {
                disable_claude: false,
                disable_codex: false,
            },
            mcp: crate::storage::McpConfig {
                disable_prompts: crate::storage::DisableOption::Bool(false),
                disable_tools: crate::storage::DisableOption::Bool(false),
            },
        };
        config.persist(&path).unwrap();
        let storage = crate::storage::Storage::new(path).unwrap();
        let server = PmxMcpServer::new(storage);

        assert!(server.is_prompt_enabled("test_prompt"));
    }

    #[test]
    fn test_is_prompt_disabled_all() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_storage");
        crate::storage::Storage::initialize(path.clone()).unwrap();

        let config = crate::storage::Config {
            agents: crate::storage::Agents {
                disable_claude: false,
                disable_codex: false,
            },
            mcp: crate::storage::McpConfig {
                disable_prompts: crate::storage::DisableOption::Bool(true),
                disable_tools: crate::storage::DisableOption::Bool(false),
            },
        };
        config.persist(&path).unwrap();
        let storage = crate::storage::Storage::new(path).unwrap();
        let server = PmxMcpServer::new(storage);

        assert!(!server.is_prompt_enabled("test_prompt"));
    }

    #[test]
    fn test_is_prompt_disabled_specific() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_storage");
        crate::storage::Storage::initialize(path.clone()).unwrap();

        let config = crate::storage::Config {
            agents: crate::storage::Agents {
                disable_claude: false,
                disable_codex: false,
            },
            mcp: crate::storage::McpConfig {
                disable_prompts: crate::storage::DisableOption::List(vec![
                    "disabled_prompt".to_string(),
                ]),
                disable_tools: crate::storage::DisableOption::Bool(false),
            },
        };
        config.persist(&path).unwrap();
        let storage = crate::storage::Storage::new(path).unwrap();
        let server = PmxMcpServer::new(storage);

        assert!(!server.is_prompt_enabled("disabled_prompt"));
        assert!(server.is_prompt_enabled("enabled_prompt"));
    }

    #[test]
    fn test_server_info() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_storage");
        let storage = crate::storage::Storage::initialize(path).unwrap();
        let server = PmxMcpServer::new(storage);

        let info = server.get_info();
        assert_eq!(info.server_info.name, "pmx-mcp-server");
        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
    }
}
