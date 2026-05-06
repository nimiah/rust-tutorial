import { Post } from "@/lib/type";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Card, CardContent, CardHeader } from "@/components/ui/card";

interface PostCardProps {
  post: Post,
}

export function PostCard({
  post,
} : PostCardProps) {
  return (
    <Card className="mb-4">
      <CardHeader className="flex items-center gap-3">
        <Avatar>
          <AvatarImage src={post.avatar} />
          <AvatarFallback>{post.author[0]}</AvatarFallback>
        </Avatar>

        <div>
          <p className="font-semibold">{post.author}</p>
          <p className="text-xs text-muted-foreground">{post.createdAt}</p>
        </div>
      </CardHeader>

      <CardContent>
        <p>{post.content}</p>

        {post.images && post.images?.length > 0 && (
          <div className="grid grid-cols-2 gap-2 mt-3">
            {post.images.map(img => (
              <div key={img} className="relative aspect-video w-full overflow-hidden rounded-md bg-muted">
                <img
                  key={img}
                  src={img}
                  alt=""
                  // className="rounded-md object-cover"
                  className="object-cover w-full h-full"
                />
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
