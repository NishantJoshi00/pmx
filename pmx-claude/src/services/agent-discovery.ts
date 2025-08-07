import * as fs from 'fs-extra';
import * as path from 'path';
import { glob } from 'glob';
import chalk from 'chalk';
import {
  AgentConfig,
  AgentRegistry,
  validateAgentConfig,
  validateAgentRegistry,
  BUILTIN_AGENTS,
  isBuiltinAgent,
  getBuiltinAgent,
} from '../types/agent-config';

export interface AgentSource {
  type: 'builtin' | 'global' | 'local' | 'pmx-profile';
  path?: string;
  priority: number; // Lower number = higher priority
}

export interface DiscoveredAgent {
  config: AgentConfig;
  source: AgentSource;
  id: string; // Unique identifier
}

export class AgentDiscoveryService {
  private currentDir: string;
  private cache: Map<string, DiscoveredAgent> = new Map();
  private cacheExpiry: number = 60000; // 1 minute
  private lastScan: number = 0;

  constructor(currentDir?: string) {
    this.currentDir = currentDir || process.cwd();
  }

  /**
   * Discover all available agents from multiple sources
   */
  async discoverAgents(options: { forceRefresh?: boolean } = {}): Promise<DiscoveredAgent[]> {
    const now = Date.now();
    
    // Use cache if still valid and not forcing refresh
    if (!options.forceRefresh && 
        this.cache.size > 0 && 
        (now - this.lastScan) < this.cacheExpiry) {
      return Array.from(this.cache.values());
    }

    this.cache.clear();
    this.lastScan = now;

    const agents: DiscoveredAgent[] = [];

    try {
      // 1. Load builtin agents (highest priority)
      agents.push(...await this.loadBuiltinAgents());

      // 2. Load global agents
      agents.push(...await this.loadGlobalAgents());

      // 3. Load local project agents
      agents.push(...await this.loadLocalAgents());

      // 4. Load PMX profile-based agents
      agents.push(...await this.loadPmxProfileAgents());

      // Sort by priority and update cache
      agents.sort((a, b) => a.source.priority - b.source.priority);
      
      for (const agent of agents) {
        this.cache.set(agent.id, agent);
      }

    } catch (error) {
      console.warn(chalk.yellow(`Warning: Agent discovery error: ${error instanceof Error ? error.message : error}`));
    }

    return agents;
  }

  /**
   * Get a specific agent by name
   */
  async getAgent(name: string): Promise<DiscoveredAgent | null> {
    const agents = await this.discoverAgents();
    return agents.find(agent => agent.config.name === name || agent.id === name) || null;
  }

  /**
   * List all available agent names
   */
  async listAgentNames(): Promise<string[]> {
    const agents = await this.discoverAgents();
    return agents.map(agent => agent.id);
  }

  /**
   * Load builtin agents
   */
  private async loadBuiltinAgents(): Promise<DiscoveredAgent[]> {
    const agents: DiscoveredAgent[] = [];

    for (const [id, partialConfig] of Object.entries(BUILTIN_AGENTS)) {
      try {
        const validatedConfig = validateAgentConfig(partialConfig);
        agents.push({
          id,
          config: validatedConfig,
          source: {
            type: 'builtin',
            priority: 100, // Lower priority than local/global
          },
        });
      } catch (error) {
        console.warn(chalk.yellow(`Warning: Invalid builtin agent '${id}': ${error}`));
      }
    }

    return agents;
  }

  /**
   * Load global agents from user's config directory
   */
  private async loadGlobalAgents(): Promise<DiscoveredAgent[]> {
    const agents: DiscoveredAgent[] = [];
    const globalConfigDir = this.getGlobalConfigDir();

    if (!await fs.pathExists(globalConfigDir)) {
      return agents;
    }

    try {
      // Look for agent registry file
      const registryPath = path.join(globalConfigDir, 'agents.json');
      if (await fs.pathExists(registryPath)) {
        const registryData = await fs.readJSON(registryPath);
        const registry = validateAgentRegistry(registryData);
        
        for (const [id, config] of Object.entries(registry.agents)) {
          agents.push({
            id,
            config,
            source: {
              type: 'global',
              path: registryPath,
              priority: 20,
            },
          });
        }
      }

      // Look for individual agent files
      const agentFiles = await glob('agents/*.json', { cwd: globalConfigDir });
      for (const agentFile of agentFiles) {
        try {
          const agentPath = path.join(globalConfigDir, agentFile);
          const agentData = await fs.readJSON(agentPath);
          const config = validateAgentConfig(agentData);
          const id = path.basename(agentFile, '.json');

          agents.push({
            id,
            config,
            source: {
              type: 'global',
              path: agentPath,
              priority: 25,
            },
          });
        } catch (error) {
          console.warn(chalk.yellow(`Warning: Invalid global agent file '${agentFile}': ${error}`));
        }
      }

    } catch (error) {
      console.warn(chalk.yellow(`Warning: Error loading global agents: ${error}`));
    }

    return agents;
  }

  /**
   * Load local project agents
   */
  private async loadLocalAgents(): Promise<DiscoveredAgent[]> {
    const agents: DiscoveredAgent[] = [];

    try {
      // Look for .pmx-claude.json in current directory
      const localConfigPath = path.join(this.currentDir, '.pmx-claude.json');
      if (await fs.pathExists(localConfigPath)) {
        const configData = await fs.readJSON(localConfigPath);
        
        if (configData.agents) {
          const registry = validateAgentRegistry(configData);
          
          for (const [id, config] of Object.entries(registry.agents)) {
            agents.push({
              id,
              config,
              source: {
                type: 'local',
                path: localConfigPath,
                priority: 10, // Highest priority
              },
            });
          }
        }
      }

      // Look for agents directory in current project
      const agentsDir = path.join(this.currentDir, '.claude-agents');
      if (await fs.pathExists(agentsDir)) {
        const agentFiles = await glob('*.json', { cwd: agentsDir });
        
        for (const agentFile of agentFiles) {
          try {
            const agentPath = path.join(agentsDir, agentFile);
            const agentData = await fs.readJSON(agentPath);
            const config = validateAgentConfig(agentData);
            const id = path.basename(agentFile, '.json');

            agents.push({
              id,
              config,
              source: {
                type: 'local',
                path: agentPath,
                priority: 15,
              },
            });
          } catch (error) {
            console.warn(chalk.yellow(`Warning: Invalid local agent file '${agentFile}': ${error}`));
          }
        }
      }

    } catch (error) {
      console.warn(chalk.yellow(`Warning: Error loading local agents: ${error}`));
    }

    return agents;
  }

  /**
   * Load PMX profiles as agents
   */
  private async loadPmxProfileAgents(): Promise<DiscoveredAgent[]> {
    const agents: DiscoveredAgent[] = [];

    try {
      // Try to find PMX configuration
      const pmxConfigDir = this.getPmxConfigDir();
      if (!await fs.pathExists(pmxConfigDir)) {
        return agents;
      }

      const repoDir = path.join(pmxConfigDir, 'repo');
      if (!await fs.pathExists(repoDir)) {
        return agents;
      }

      // Find all .md files in the repo directory
      const profileFiles = await glob('**/*.md', { cwd: repoDir });
      
      for (const profileFile of profileFiles) {
        try {
          const profilePath = path.join(repoDir, profileFile);
          const content = await fs.readFile(profilePath, 'utf-8');
          
          // Skip empty files
          if (!content.trim()) continue;
          
          const profileName = profileFile.replace(/\.md$/, '').replace(/\//g, '-');
          const displayName = profileFile.replace(/\.md$/, '').replace(/\//g, '/');
          
          const agentConfigData = {
            name: `PMX Profile: ${displayName}`,
            description: `Agent using PMX profile '${displayName}' as system prompt`,
            systemPrompt: content,
            filePatterns: ['**/*.{js,ts,jsx,tsx,py,rs,go,java,cpp,c,h,php,rb}'],
            category: 'pmx-profile',
            tags: ['pmx', 'profile'],
            pmxProfile: displayName,
            capabilities: {
              canModifyFiles: false,
              canCreateFiles: false,
              canExecuteCommands: false,
              requiresConfirmation: true,
            },
          };

          const agentConfig = validateAgentConfig(agentConfigData);

          agents.push({
            id: `pmx-${profileName}`,
            config: agentConfig,
            source: {
              type: 'pmx-profile',
              path: profilePath,
              priority: 30, // Lower priority than explicit agents
            },
          });

        } catch (error) {
          console.warn(chalk.yellow(`Warning: Error loading PMX profile '${profileFile}': ${error}`));
        }
      }

    } catch (error) {
      console.warn(chalk.yellow(`Warning: Error loading PMX profiles: ${error}`));
    }

    return agents;
  }

  /**
   * Get global configuration directory
   */
  private getGlobalConfigDir(): string {
    const homeDir = require('os').homedir();
    const xdgConfigHome = process.env.XDG_CONFIG_HOME;
    
    if (xdgConfigHome) {
      return path.join(xdgConfigHome, 'pmx-claude');
    }
    
    return path.join(homeDir, '.config', 'pmx-claude');
  }

  /**
   * Get PMX configuration directory
   */
  private getPmxConfigDir(): string {
    const homeDir = require('os').homedir();
    const xdgConfigHome = process.env.XDG_CONFIG_HOME;
    
    if (xdgConfigHome) {
      return path.join(xdgConfigHome, 'pmx');
    }
    
    return path.join(homeDir, '.config', 'pmx');
  }

  /**
   * Initialize global configuration directory with default agents
   */
  async initializeGlobalConfig(): Promise<void> {
    const globalConfigDir = this.getGlobalConfigDir();
    
    await fs.ensureDir(globalConfigDir);
    await fs.ensureDir(path.join(globalConfigDir, 'agents'));
    
    const registryPath = path.join(globalConfigDir, 'agents.json');
    
    if (!await fs.pathExists(registryPath)) {
      const defaultRegistry: AgentRegistry = {
        version: '1.0.0',
        agents: {},
        globalSettings: {
          defaultModel: 'claude-3-5-sonnet-20241022',
          defaultMaxTokens: 4000,
          defaultTemperature: 0.1,
        },
      };
      
      await fs.writeJSON(registryPath, defaultRegistry, { spaces: 2 });
    }
  }

  /**
   * Initialize local project configuration
   */
  async initializeLocalConfig(): Promise<void> {
    const localConfigPath = path.join(this.currentDir, '.pmx-claude.json');
    
    if (!await fs.pathExists(localConfigPath)) {
      const defaultConfig: AgentRegistry = {
        version: '1.0.0',
        agents: {},
        globalSettings: {},
      };
      
      await fs.writeJSON(localConfigPath, defaultConfig, { spaces: 2 });
    }
    
    // Create agents directory
    const agentsDir = path.join(this.currentDir, '.claude-agents');
    await fs.ensureDir(agentsDir);
    
    // Create .gitignore entry
    const gitignorePath = path.join(this.currentDir, '.gitignore');
    if (await fs.pathExists(gitignorePath)) {
      const gitignoreContent = await fs.readFile(gitignorePath, 'utf-8');
      if (!gitignoreContent.includes('.pmx-claude.json')) {
        await fs.appendFile(gitignorePath, '\n# PMX Claude configuration\n.pmx-claude.json\n.claude-agents/\n');
      }
    }
  }
}