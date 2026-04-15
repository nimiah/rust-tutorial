export default function Articles() {
  const articles: { name: string }[] = [
    {
      name: "Test1",
    },
    {
      name: "Test2",
    },
    {
      name: "Test3",
    },
  ];

  return (
    <div>
      Articles
      <div className="flex flex-col gap-2">
        {articles.map((article) => (
          <span>{article.name}</span>
        ))}
      </div>
    </div>
  );
}
