declare module 'markdown-it' {
  interface MarkdownIt {
    render(src: string): string
    renderInline(src: string): string
  }

  interface MarkdownItConstructor {
    new (options?: any): MarkdownIt
  }

  const MarkdownIt: MarkdownItConstructor
  export = MarkdownIt
}
