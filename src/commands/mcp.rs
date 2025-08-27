use anyhow::Result;
use rmcp::{
    RoleServer, ServerHandler, ServiceExt,
    model::{ErrorData as McpError, *},
    service::RequestContext,
};
use tokio::io::{stdin, stdout};
use serde_json::Value;

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

    /// Extract argument templates from prompt content using <{{variable}}> pattern
    fn extract_arguments_from_content(&self, content: &str) -> Vec<PromptArgument> {
        use regex::Regex;
        
        // Pattern matches <{{VARIABLE_NAME}}> where VARIABLE_NAME can contain letters, numbers, underscores
        let re = Regex::new(r"<\{\{([A-Za-z_][A-Za-z0-9_]*)\}\}>").unwrap();
        let mut arguments = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for cap in re.captures_iter(content) {
            if let Some(var_name) = cap.get(1) {
                let name = var_name.as_str().to_string();
                // Avoid duplicates
                if seen.insert(name.clone()) {
                    arguments.push(PromptArgument {
                        name: name.clone(),
                        description: Some(format!("Value for {}", name)),
                        required: Some(true),
                    });
                }
            }
        }
        
        arguments
    }

    /// Replace argument placeholders in content with provided values
    fn substitute_arguments(&self, content: &str, arguments: &Option<JsonObject>) -> String {
        let Some(args) = arguments else {
            return content.to_string();
        };
        
        use regex::Regex;
        let re = Regex::new(r"<\{\{([A-Za-z_][A-Za-z0-9_]*)\}\}>").unwrap();
        
        re.replace_all(content, |caps: &regex::Captures| {
            let var_name = &caps[1];
            match args.get(var_name) {
                Some(Value::String(s)) => s.clone(),
                Some(other) => other.to_string().trim_matches('"').to_string(),
                None => caps.get(0).unwrap().as_str().to_string(), // Keep original if not found
            }
        }).to_string()
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
                // Read the content to extract arguments
                let arguments = match self.storage.get_content(&profile) {
                    Ok(content) => {
                        let extracted_args = self.extract_arguments_from_content(&content);
                        if extracted_args.is_empty() {
                            None
                        } else {
                            Some(extracted_args)
                        }
                    }
                    Err(_) => None, // If we can't read the content, don't include arguments
                };

                prompts.push(Prompt::new(
                    &profile,
                    Some(&format!("System prompt: {profile}")),
                    arguments,
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
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        if !self.is_prompt_enabled(&name) {
            return Err(McpError::invalid_params("Prompt is disabled", None));
        }

        let content = self
            .storage
            .get_content(&name)
            .map_err(|e| McpError::invalid_params(format!("Prompt not found: {e}"), None))?;

        // Substitute arguments in the content
        let processed_content = self.substitute_arguments(&content, &arguments);

        Ok(GetPromptResult {
            description: None,
            messages: vec![PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(processed_content),
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
    use serde_json::json;

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
            extensions: crate::storage::ExtensionsConfig::default(),
        };
        config.persist(&path).unwrap();
        let storage = crate::storage::Storage::new(path).unwrap();
        let server = PmxMcpServer::new(storage);

        assert!(server.is_prompt_enabled("test_prompt"));
    }

    #[test]
    fn test_extract_arguments_from_content() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_storage");
        let storage = crate::storage::Storage::initialize(path).unwrap();
        let server = PmxMcpServer::new(storage);

        // Test extracting single argument
        let content1 = "Please visit <{{URL}}> for more information.";
        let args1 = server.extract_arguments_from_content(content1);
        assert_eq!(args1.len(), 1);
        assert_eq!(args1[0].name, "URL");
        assert_eq!(args1[0].description, Some("Value for URL".to_string()));
        assert_eq!(args1[0].required, Some(true));

        // Test extracting multiple arguments
        let content2 = "Connect to <{{HOST}}> on port <{{PORT}}> using <{{PROTOCOL}}>";
        let args2 = server.extract_arguments_from_content(content2);
        assert_eq!(args2.len(), 3);
        let names: Vec<&str> = args2.iter().map(|a| a.name.as_str()).collect();
        assert!(names.contains(&"HOST"));
        assert!(names.contains(&"PORT"));
        assert!(names.contains(&"PROTOCOL"));

        // Test no arguments
        let content3 = "This is a simple prompt without variables.";
        let args3 = server.extract_arguments_from_content(content3);
        assert_eq!(args3.len(), 0);

        // Test duplicate arguments (should be deduplicated)
        let content4 = "Use <{{URL}}> to access <{{URL}}> again.";
        let args4 = server.extract_arguments_from_content(content4);
        assert_eq!(args4.len(), 1);
        assert_eq!(args4[0].name, "URL");

        // Test invalid patterns (should not match)
        let content5 = "Invalid patterns: <{URL}> and {{URL}} and <URL>";
        let args5 = server.extract_arguments_from_content(content5);
        assert_eq!(args5.len(), 0);
    }

    #[test]
    fn test_substitute_arguments() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_storage");
        let storage = crate::storage::Storage::initialize(path).unwrap();
        let server = PmxMcpServer::new(storage);

        // Test basic substitution
        let content = "Please visit <{{URL}}> for more information.";
        let mut args = serde_json::Map::new();
        args.insert("URL".to_string(), json!("https://example.com"));
        let result = server.substitute_arguments(content, &Some(args));
        assert_eq!(result, "Please visit https://example.com for more information.");

        // Test multiple substitutions
        let content2 = "Connect to <{{HOST}}> on port <{{PORT}}>"; 
        let mut args2 = serde_json::Map::new();
        args2.insert("HOST".to_string(), json!("localhost"));
        args2.insert("PORT".to_string(), json!(8080));
        let result2 = server.substitute_arguments(content2, &Some(args2));
        assert_eq!(result2, "Connect to localhost on port 8080");

        // Test missing arguments (should keep original)
        let content3 = "Use <{{MISSING}}> value.";
        let args3 = serde_json::Map::new();
        let result3 = server.substitute_arguments(content3, &Some(args3));
        assert_eq!(result3, "Use <{{MISSING}}> value.");

        // Test no arguments provided
        let content4 = "Use <{{URL}}> value.";
        let result4 = server.substitute_arguments(content4, &None);
        assert_eq!(result4, "Use <{{URL}}> value.");
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
            extensions: crate::storage::ExtensionsConfig::default(),
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
            extensions: crate::storage::ExtensionsConfig::default(),
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
