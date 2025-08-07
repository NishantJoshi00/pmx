import { z } from 'zod';

// Agent configuration schema for validation
export const AgentConfigSchema = z.object({
  name: z.string().min(1, 'Agent name is required'),
  version: z.string().optional().default('1.0.0'),
  description: z.string().min(1, 'Agent description is required'),
  author: z.string().optional(),
  
  // Core agent behavior
  systemPrompt: z.string().min(1, 'System prompt is required'),
  
  // File processing configuration
  filePatterns: z.array(z.string()).optional().default(['**/*']),
  excludePatterns: z.array(z.string()).optional().default([
    '**/node_modules/**',
    '**/target/**', 
    '**/dist/**',
    '**/.git/**',
    '**/build/**',
    '**/.next/**',
    '**/coverage/**'
  ]),
  
  // Model parameters
  model: z.string().optional().default('claude-3-5-sonnet-20241022'),
  maxTokens: z.number().min(1).max(8192).optional().default(4000),
  temperature: z.number().min(0).max(1).optional().default(0.1),
  
  // File size and count limits
  maxFileSize: z.number().optional().default(100 * 1024), // 100KB
  maxFiles: z.number().optional().default(20),
  
  // Agent capabilities
  capabilities: z.object({
    canModifyFiles: z.boolean().optional().default(false),
    canCreateFiles: z.boolean().optional().default(false),
    canExecuteCommands: z.boolean().optional().default(false),
    requiresConfirmation: z.boolean().optional().default(true),
  }).optional().default(() => ({
    canModifyFiles: false,
    canCreateFiles: false,
    canExecuteCommands: false,
    requiresConfirmation: true,
  })),
  
  // Output configuration
  outputFormat: z.enum(['text', 'markdown', 'json']).optional().default('text'),
  
  // Context enhancement
  includeProjectInfo: z.boolean().optional().default(true),
  includeGitInfo: z.boolean().optional().default(true),
  
  // PMX integration
  pmxProfile: z.string().optional(), // Name of PMX profile to use as base system prompt
  
  // Custom metadata
  tags: z.array(z.string()).optional().default([]),
  category: z.string().optional(),
});

export type AgentConfig = z.infer<typeof AgentConfigSchema>;

// Agent registry configuration
export const AgentRegistrySchema = z.object({
  version: z.string().default('1.0.0'),
  agents: z.record(z.string(), AgentConfigSchema),
  globalSettings: z.object({
    defaultModel: z.string().optional(),
    defaultMaxTokens: z.number().optional(),
    defaultTemperature: z.number().optional(),
  }).optional().default({}),
});

export type AgentRegistry = z.infer<typeof AgentRegistrySchema>;

// Built-in agent definitions (partial configs that will be validated and filled with defaults)
export const BUILTIN_AGENTS: Record<string, Partial<AgentConfig> & { name: string; description: string; systemPrompt: string }> = {
  'code-review': {
    name: 'Code Review',
    description: 'Analyzes code quality, style, and potential issues',
    systemPrompt: `You are a senior code reviewer with expertise across multiple programming languages and frameworks. 

Analyze the provided code for:
- Code quality and adherence to best practices
- Potential bugs, security vulnerabilities, and performance issues
- Code style consistency and readability
- Architecture and design patterns
- Documentation quality and completeness
- Test coverage and testing patterns

Provide constructive feedback with:
- Specific examples and line references
- Actionable recommendations
- Priority levels (critical, important, suggestion)
- Alternative approaches where applicable

Focus on being helpful and educational rather than just critical.`,
    filePatterns: ['**/*.{js,ts,jsx,tsx,py,rs,go,java,cpp,c,h,php,rb,swift,kt}'],
    category: 'analysis',
    tags: ['code-quality', 'review', 'best-practices'],
    capabilities: {
      canModifyFiles: false,
      canCreateFiles: false,
      canExecuteCommands: false,
      requiresConfirmation: false,
    },
  },
  
  'documentation': {
    name: 'Documentation Generator',
    description: 'Generates or improves project documentation',
    systemPrompt: `You are a technical documentation expert specializing in creating clear, comprehensive, and user-friendly documentation.

Based on the provided code and project structure:
- Generate or improve README files with proper sections (installation, usage, API, examples)
- Create API documentation with clear parameter descriptions and examples
- Document code functions and classes with appropriate detail
- Suggest code comments where they would improve understanding
- Create getting-started guides and tutorials
- Ensure documentation is up-to-date with the current codebase

Focus on:
- Clarity and accessibility for different skill levels
- Practical examples and use cases
- Proper formatting and structure
- Completeness without verbosity`,
    filePatterns: [
      '**/*.{js,ts,jsx,tsx,py,rs,go,java}',
      '**/README.md',
      '**/package.json',
      '**/Cargo.toml',
      '**/pyproject.toml',
      '**/go.mod'
    ],
    category: 'documentation',
    tags: ['docs', 'readme', 'api-docs'],
    capabilities: {
      canModifyFiles: true,
      canCreateFiles: true,
      canExecuteCommands: false,
      requiresConfirmation: true,
    },
  },
  
  'test-generation': {
    name: 'Test Generator',
    description: 'Creates comprehensive test files for existing code',
    systemPrompt: `You are a test engineering expert with deep knowledge of testing frameworks and best practices across multiple languages.

Analyze the provided code and generate:
- Comprehensive unit tests with good coverage
- Integration tests where appropriate
- Edge case and error scenario tests
- Mock implementations for external dependencies
- Test setup and teardown procedures
- Clear test descriptions and assertions

Follow testing best practices:
- Use appropriate testing frameworks for the language/project
- Write readable and maintainable tests
- Include both positive and negative test cases
- Test error handling and boundary conditions
- Provide clear test organization and structure
- Include performance tests where relevant

Generate complete, runnable test files that follow the project's existing patterns.`,
    filePatterns: ['**/*.{js,ts,jsx,tsx,py,rs,go,java,php,rb,swift,kt}'],
    excludePatterns: [
      '**/node_modules/**',
      '**/target/**',
      '**/dist/**',
      '**/.git/**',
      '**/build/**',
      '**/*test*',
      '**/*spec*',
      '**/tests/**',
      '**/test/**'
    ],
    category: 'testing',
    tags: ['testing', 'unit-tests', 'coverage'],
    capabilities: {
      canModifyFiles: false,
      canCreateFiles: true,
      canExecuteCommands: false,
      requiresConfirmation: true,
    },
  },
  
  'refactor': {
    name: 'Code Refactoring Assistant',
    description: 'Suggests and implements code refactoring improvements',
    systemPrompt: `You are a refactoring expert with deep understanding of clean code principles and design patterns.

Analyze the provided code and suggest refactoring improvements:
- Extract methods/functions to reduce complexity
- Eliminate code duplication (DRY principle)
- Improve naming conventions for clarity
- Simplify complex conditional logic
- Apply appropriate design patterns
- Optimize performance where beneficial
- Improve code organization and structure
- Enhance error handling and robustness

For each suggestion:
- Explain the current issue or improvement opportunity
- Provide the refactored code with clear before/after examples
- Explain the benefits of the change
- Ensure the refactoring maintains the same functionality
- Consider backward compatibility and breaking changes`,
    filePatterns: ['**/*.{js,ts,jsx,tsx,py,rs,go,java,cpp,c,h,php,rb,swift,kt}'],
    category: 'improvement',
    tags: ['refactoring', 'clean-code', 'optimization'],
    capabilities: {
      canModifyFiles: true,
      canCreateFiles: false,
      canExecuteCommands: false,
      requiresConfirmation: true,
    },
    temperature: 0.2, // Slightly higher for more creative refactoring suggestions
  },
};

// Validation functions
export function validateAgentConfig(config: unknown): AgentConfig {
  return AgentConfigSchema.parse(config);
}

export function validateAgentRegistry(registry: unknown): AgentRegistry {
  return AgentRegistrySchema.parse(registry);
}

// Helper functions
export function isBuiltinAgent(name: string): boolean {
  return name in BUILTIN_AGENTS;
}

export function getBuiltinAgent(name: string): AgentConfig | undefined {
  const builtinConfig = BUILTIN_AGENTS[name];
  if (!builtinConfig) return undefined;
  
  try {
    return validateAgentConfig(builtinConfig);
  } catch {
    return undefined;
  }
}