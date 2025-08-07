import chalk from 'chalk';

export async function showStatus(): Promise<void> {
  console.log(chalk.blue('🔍 Claude Agent Provider Status (Claude Code Compatible)\n'));

  // Check Anthropic API
  const anthropicKey = process.env.ANTHROPIC_API_KEY;
  if (anthropicKey) {
    console.log(chalk.green('✅ Anthropic API: Available'));
    console.log(chalk.gray(`   Key: ${anthropicKey.substring(0, 8)}...${anthropicKey.substring(anthropicKey.length - 4)}`));
  } else {
    console.log(chalk.red('❌ Anthropic API: Not configured'));
    console.log(chalk.gray('   Set ANTHROPIC_API_KEY environment variable'));
  }

  console.log();

  // Check Claude Code Vertex AI variables
  const useVertex = process.env.CLAUDE_CODE_USE_VERTEX;
  const vertexProjectId = process.env.ANTHROPIC_VERTEX_PROJECT_ID;
  const vertexRegion = process.env.CLOUD_ML_REGION;

  if (useVertex && vertexProjectId) {
    console.log(chalk.green('✅ Vertex AI (Claude Code Compatible): Available'));
    console.log(chalk.gray(`   CLAUDE_CODE_USE_VERTEX: ${useVertex}`));
    console.log(chalk.gray(`   Project: ${vertexProjectId}`));
    console.log(chalk.gray(`   Region: ${vertexRegion || 'us-east5 (default)'}`));
  } else {
    console.log(chalk.red('❌ Vertex AI (Claude Code Compatible): Not configured'));
    console.log(chalk.gray('   Set Claude Code environment variables:'));
    console.log(chalk.gray('   • CLAUDE_CODE_USE_VERTEX=true'));
    console.log(chalk.gray('   • ANTHROPIC_VERTEX_PROJECT_ID=your-project-id'));
    console.log(chalk.gray('   • CLOUD_ML_REGION=your-region (optional)'));
  }

  console.log();

  // Show recommendation
  if (anthropicKey && (useVertex && vertexProjectId)) {
    console.log(chalk.cyan('💡 Both providers available! Use --provider flag to choose or let auto-detection decide.'));
    console.log(chalk.cyan('💡 Auto-detection will prefer Vertex AI when Claude Code variables are set.'));
  } else if (anthropicKey) {
    console.log(chalk.cyan('💡 Using Anthropic API. For Vertex AI, set Claude Code variables:'));
    console.log(chalk.gray('   export CLAUDE_CODE_USE_VERTEX=true'));
    console.log(chalk.gray('   export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id'));
  } else if (useVertex && vertexProjectId) {
    console.log(chalk.cyan('💡 Using Vertex AI via Claude Code configuration.'));
  } else {
    console.log(chalk.yellow('⚠️  No credentials configured! Set up either:'));
    console.log(chalk.gray('   • ANTHROPIC_API_KEY for direct Anthropic API'));
    console.log(chalk.gray('   • Claude Code Vertex variables (see above)'));
  }
}