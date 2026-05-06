export type Post = {
  id: string
  author: string
  avatar?: string
  content: string
  images?: string[]
  createdAt: string
}

export type FeedPage = {
  items: Post[]
  nextCursor: string | null
}



