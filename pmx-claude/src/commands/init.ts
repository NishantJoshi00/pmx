import chalk from 'chalk';
import { AgentDiscoveryService } from '../services/agent-discovery';

export async function initializeProject(options: { global?: boolean } = {}): Promise<void> {
  console.log(chalk.blue('🚀 Initializing Claude agent configuration...\n'));

  try {
    const discoveryService = new AgentDiscoveryService(process.cwd());

    if (options.global) {
      console.log(chalk.blue('Setting up global agent configuration...'));
      await discoveryService.initializeGlobalConfig();
      console.log(chalk.green('✅ Global configuration initialized'));
      console.log(chalk.gray('   Configuration directory: ~/.config/pmx-claude/'));
      console.log(chalk.gray('   You can now create custom agents in the agents/ directory'));
    } else {
      console.log(chalk.blue('Setting up local project configuration...'));
      await discoveryService.initializeLocalConfig();
      console.log(chalk.green('✅ Local project configuration initialized'));
      console.log(chalk.gray('   Created: .pmx-claude.json'));
      console.log(chalk.gray('   Created: .claude-agents/ directory'));
      console.log(chalk.gray('   Updated: .gitignore (if it exists)'));
    }

    console.log(chalk.cyan('\n💡 Next steps:'));
    if (options.global) {
      console.log(chalk.gray('   • Create custom agents in ~/.config/pmx-claude/agents/'));
      console.log(chalk.gray('   • Edit ~/.config/pmx-claude/agents.json to register agents'));
    } else {
      console.log(chalk.gray('   • Create custom agents in .claude-agents/ directory'));
      console.log(chalk.gray('   • Edit .pmx-claude.json to configure project-specific agents'));
    }
    console.log(chalk.gray('   • Use "pmx claude list-agents" to see available agents'));
    console.log(chalk.gray('   • Use "pmx claude run <agent-name>" to execute agents'));

  } catch (error) {
    throw new Error(`Failed to initialize configuration: ${error instanceof Error ? error.message : error}`);
  }
}