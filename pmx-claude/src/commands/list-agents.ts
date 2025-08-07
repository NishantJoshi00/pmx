import chalk from 'chalk';
import { AgentRunner } from '../agent-runner';

export async function listAgents(options: { verbose?: boolean } = {}): Promise<void> {
  console.log(chalk.blue('🤖 Available Claude Agents\n'));

  try {
    const runner = new AgentRunner();
    const discoveredAgents = await runner.listAvailableAgents();

    if (discoveredAgents.length === 0) {
      console.log(chalk.yellow('No agents found.'));
      console.log(chalk.gray('💡 Try running "pmx claude init" to set up agent configuration'));
      return;
    }

    // Group agents by source type
    const groupedAgents = discoveredAgents.reduce((groups, agent) => {
      const sourceType = agent.source.type;
      if (!groups[sourceType]) {
        groups[sourceType] = [];
      }
      groups[sourceType].push(agent);
      return groups;
    }, {} as Record<string, typeof discoveredAgents>);

    // Display agents by source
    const sourceLabels = {
      'local': '🏠 Local Project Agents',
      'global': '🌐 Global Agents', 
      'builtin': '⚡ Built-in Agents',
      'pmx-profile': '📝 PMX Profile Agents'
    };

    for (const [sourceType, agents] of Object.entries(groupedAgents)) {
      if (agents.length === 0) continue;
      
      console.log(chalk.blue(`${sourceLabels[sourceType as keyof typeof sourceLabels] || sourceType}:`));
      
      agents.forEach((agent, index) => {
        console.log(chalk.green(`  ${index + 1}. ${agent.id}`));
        console.log(chalk.gray(`     ${agent.config.description}`));
        
        if (options.verbose) {
          console.log(chalk.gray(`     Source: ${agent.source.path || 'Built-in'}`));
          if (agent.config.category) {
            console.log(chalk.gray(`     Category: ${agent.config.category}`));
          }
          if (agent.config.tags && agent.config.tags.length > 0) {
            console.log(chalk.gray(`     Tags: ${agent.config.tags.join(', ')}`));
          }
        }
        
        console.log(chalk.blue(`     Usage: pmx claude run ${agent.id}`));
        console.log();
      });
    }

    console.log(chalk.cyan('💡 Tip: Use "pmx claude run <agent-id>" to execute an agent'));
    console.log(chalk.cyan('💡 Use "--dry-run" flag to see what an agent would do without executing'));
    console.log(chalk.cyan('💡 Use "--verbose" flag to see more agent details'));
    
    if (!groupedAgents['local']) {
      console.log(chalk.cyan('💡 Create local agents with "pmx claude init" in your project'));
    }
  } catch (error) {
    throw new Error(`Failed to list agents: ${error instanceof Error ? error.message : error}`);
  }
}