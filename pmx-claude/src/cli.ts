#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import { AgentRunner } from './agent-runner';
import { listAgents } from './commands/list-agents';
import { runAgent } from './commands/run-agent';
import { showStatus } from './commands/status';
import { initializeProject } from './commands/init';

const program = new Command();

program
  .name('pmx-claude')
  .description('Claude Code agent runner for PMX')
  .version('1.0.0');

program
  .command('list-agents')
  .alias('list')
  .description('List available Claude agents')
  .option('-v, --verbose', 'Show detailed agent information')
  .action(async (options) => {
    try {
      await listAgents(options);
    } catch (error) {
      console.error(chalk.red('Error listing agents:'), error instanceof Error ? error.message : error);
      process.exit(1);
    }
  });

program
  .command('run <agent-name>')
  .description('Run a specific Claude agent on the current directory')
  .option('-v, --verbose', 'Enable verbose output')
  .option('--dry-run', 'Show what the agent would do without executing')
  .option('--config <path>', 'Path to custom agent configuration file')
  .option('--provider <provider>', 'AI provider to use: anthropic, vertex, or auto (default: auto)', 'auto')
  .action(async (agentName: string, options) => {
    try {
      await runAgent(agentName, options);
    } catch (error) {
      console.error(chalk.red(`Error running agent '${agentName}':`), error instanceof Error ? error.message : error);
      process.exit(1);
    }
  });

program
  .command('status')
  .description('Show provider configuration and credential status')
  .action(async () => {
    try {
      await showStatus();
    } catch (error) {
      console.error(chalk.red('Error checking status:'), error instanceof Error ? error.message : error);
      process.exit(1);
    }
  });

program
  .command('init')
  .description('Initialize agent configuration for current project')
  .option('--global', 'Initialize global configuration instead of local')
  .action(async (options) => {
    try {
      await initializeProject(options);
    } catch (error) {
      console.error(chalk.red('Error initializing configuration:'), error instanceof Error ? error.message : error);
      process.exit(1);
    }
  });

// Error handling for unknown commands
program.on('command:*', () => {
  console.error(chalk.red(`Unknown command: ${program.args.join(' ')}`));
  console.log(chalk.blue('Run --help for available commands'));
  process.exit(1);
});

// Parse command line arguments
program.parse();

// Show help if no command provided
if (!process.argv.slice(2).length) {
  program.outputHelp();
}