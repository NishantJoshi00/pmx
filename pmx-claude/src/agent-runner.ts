import Anthropic from '@anthropic-ai/sdk';
import AnthropicVertex from '@anthropic-ai/vertex-sdk';
import * as fs from 'fs-extra';
import * as path from 'path';
import { glob } from 'glob';
import chalk from 'chalk';
import { AgentDiscoveryService, DiscoveredAgent } from './services/agent-discovery';
import { AgentConfig } from './types/agent-config';


export interface RunOptions {
  verbose?: boolean;
  dryRun?: boolean;
  config?: string;
  provider?: 'anthropic' | 'vertex' | 'auto';
}

export type AnthropicClient = Anthropic | AnthropicVertex;

export class AgentRunner {
  private client: AnthropicClient;
  private currentDir: string;
  private provider: string;
  private discoveryService: AgentDiscoveryService;

  constructor(options: { provider?: 'anthropic' | 'vertex' | 'auto' } = {}) {
    this.currentDir = process.cwd();
    const { client, provider } = this.initializeClient(options.provider || 'auto');
    this.client = client;
    this.provider = provider;
    this.discoveryService = new AgentDiscoveryService(this.currentDir);
  }

  private initializeClient(providerPreference: 'anthropic' | 'vertex' | 'auto'): { client: AnthropicClient; provider: string } {
    // Check Claude Code environment variables for consistency
    const useVertex = process.env.CLAUDE_CODE_USE_VERTEX;
    const vertexProjectId = process.env.ANTHROPIC_VERTEX_PROJECT_ID;
    const vertexRegion = process.env.CLOUD_ML_REGION;

    // Auto-detect based on available credentials (Claude Code compatible)
    if (providerPreference === 'auto') {
      // Use Vertex AI if Claude Code Vertex variables are set
      if (useVertex && vertexProjectId) {
        try {
          const client = new AnthropicVertex({
            projectId: vertexProjectId,
            region: vertexRegion || 'us-east5',
          });
          return { client, provider: 'vertex' };
        } catch (error) {
          // Fall back to Anthropic API
        }
      }

      // Fall back to Anthropic API
      const apiKey = process.env.ANTHROPIC_API_KEY;
      if (!apiKey) {
        throw new Error('No valid credentials found. Set either ANTHROPIC_API_KEY or Claude Code Vertex variables (CLAUDE_CODE_USE_VERTEX, ANTHROPIC_VERTEX_PROJECT_ID)');
      }
      
      const client = new Anthropic({ apiKey });
      return { client, provider: 'anthropic' };
    }

    // Explicit provider selection
    if (providerPreference === 'vertex') {
      if (!vertexProjectId) {
        throw new Error('Vertex AI requires ANTHROPIC_VERTEX_PROJECT_ID environment variable (Claude Code compatible)');
      }
      
      const client = new AnthropicVertex({
        projectId: vertexProjectId,
        region: vertexRegion || 'us-east5',
      });
      return { client, provider: 'vertex' };
    }

    // Anthropic API
    const apiKey = process.env.ANTHROPIC_API_KEY;
    if (!apiKey) {
      throw new Error('ANTHROPIC_API_KEY environment variable is required');
    }
    
    const client = new Anthropic({ apiKey });
    return { client, provider: 'anthropic' };
  }

  async runAgent(agentName: string, options: RunOptions = {}): Promise<void> {
    if (options.verbose) {
      console.log(chalk.blue(`🤖 Running agent: ${agentName}`));
      console.log(chalk.gray(`📁 Working directory: ${this.currentDir}`));
      console.log(chalk.gray(`🔗 Provider: ${this.provider}`));
    }

    const discoveredAgent = await this.discoveryService.getAgent(agentName);
    if (!discoveredAgent) {
      throw new Error(`Agent '${agentName}' not found. Use 'list-agents' to see available agents.`);
    }
    const agent = discoveredAgent.config;
    const context = await this.gatherContext(agent);
    
    if (options.dryRun) {
      console.log(chalk.yellow('🏃 DRY RUN MODE - No changes will be made'));
      console.log(chalk.blue('Agent:'), agent.name);
      console.log(chalk.blue('Description:'), agent.description);
      console.log(chalk.blue('Provider:'), this.provider);
      console.log(chalk.blue('Context files:'), context.files.length);
      return;
    }

    const prompt = this.buildPrompt(agent, context);
    
    if (options.verbose) {
      console.log(chalk.blue('💭 Sending request to Claude...'));
    }

    const response = await this.client.messages.create({
      model: 'claude-3-5-sonnet-20241022',
      max_tokens: agent.maxTokens || 4000,
      temperature: agent.temperature || 0.1,
      messages: [{
        role: 'user',
        content: prompt
      }]
    });

    if (response.content[0].type === 'text') {
      console.log(chalk.green('🎯 Agent Response:'));
      console.log(response.content[0].text);
    }
  }


  private async gatherContext(agent: AgentConfig): Promise<{ files: Array<{ path: string; content: string }> }> {
    const files: Array<{ path: string; content: string }> = [];
    const patterns = agent.filePatterns || ['**/*'];
    const excludePatterns = agent.excludePatterns || [
      '**/node_modules/**', 
      '**/target/**', 
      '**/dist/**', 
      '**/.git/**', 
      '**/build/**'
    ];

    for (const pattern of patterns) {
      const matchedFiles = await glob(pattern, {
        cwd: this.currentDir,
        ignore: excludePatterns,
        nodir: true
      });

      for (const filePath of matchedFiles.slice(0, agent.maxFiles || 20)) {
        try {
          const fullPath = path.join(this.currentDir, filePath);
          const stat = await fs.stat(fullPath);
          
          // Skip large files
          if (stat.size > (agent.maxFileSize || 100 * 1024)) continue;
          
          const content = await fs.readFile(fullPath, 'utf-8');
          files.push({ path: filePath, content });
        } catch (error) {
          // Skip files that can't be read
          continue;
        }
      }
    }

    return { files };
  }

  private buildPrompt(agent: AgentConfig, context: { files: Array<{ path: string; content: string }> }): string {
    let prompt = agent.systemPrompt + '\n\n';
    
    prompt += `## Project Context\n`;
    prompt += `Working directory: ${this.currentDir}\n`;
    prompt += `Files analyzed: ${context.files.length}\n\n`;

    prompt += `## Code Files\n\n`;
    
    for (const file of context.files) {
      prompt += `### File: ${file.path}\n`;
      prompt += '```\n';
      prompt += file.content;
      prompt += '\n```\n\n';
    }

    prompt += `## Instructions\n`;
    prompt += `Please analyze the above code and provide your expert assessment based on your role as: ${agent.name}.\n`;
    prompt += `Focus on actionable insights and specific recommendations.`;

    return prompt;
  }

  async listAvailableAgents(): Promise<DiscoveredAgent[]> {
    return await this.discoveryService.discoverAgents();
  }
}