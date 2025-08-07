import chalk from 'chalk';
import { AgentRunner, RunOptions } from '../agent-runner';

export async function runAgent(agentName: string, options: RunOptions): Promise<void> {
  if (!agentName) {
    throw new Error('Agent name is required');
  }

  console.log(chalk.blue(`🚀 Initializing agent: ${agentName}`));

  // Validate current directory
  const currentDir = process.cwd();
  if (options.verbose) {
    console.log(chalk.gray(`📁 Working in: ${currentDir}`));
  }

  try {
    const runner = new AgentRunner({ provider: options.provider || 'auto' });
    await runner.runAgent(agentName, options);
    
    if (!options.dryRun) {
      console.log(chalk.green('✅ Agent execution completed'));
    }
  } catch (error) {
    if (error instanceof Error && error.message.includes('not found')) {
      console.log(chalk.red(`❌ Agent '${agentName}' not found`));
      console.log(chalk.yellow('💡 Use "pmx claude list-agents" to see available agents'));
    }
    throw error;
  }
}