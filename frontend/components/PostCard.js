import Link from 'next/link';
import { useMemo } from 'react';

// Component for displaying individual tags
const Tag = ({ tag }) => (
    <span className="bg-blue-100 text-blue-800 text-xs font-medium mr-2 px-2.5 py-0.5 rounded">
        {tag}
    </span>
);

const PostCard = ({ post }) => {
    // Memoize formatted date for efficiency
    const formattedDate = useMemo(() => new Date(post.created_at).toLocaleDateString(), [post.created_at]);

    return (
        <div className="border p-6 rounded-lg shadow hover:shadow-lg transition-shadow duration-300 bg-white">
            <div className="flex justify-between items-start">
                <div className="flex-1">
                    {/* Featured post badge */}
                    {post.feature && (
                        <span className="bg-yellow-200 text-yellow-800 text-xs font-semibold px-2.5 py-0.5 rounded mb-2 inline-block">
                            精选文章
                        </span>
                    )}
                    <h3 className="text-2xl font-bold mb-2">{post.title}</h3>
                    <p className="text-gray-700 mb-4">{post.digest}</p>

                    {/* Render tags using Tag component */}
                    {post.tags && post.tags.length > 0 && (
                        <div className="flex flex-wrap mb-4">
                            {post.tags.map(tag => (
                                <Tag key={tag} tag={tag} />
                            ))}
                        </div>
                    )}
                </div>

                {/* Author information with link fix */}
                <div className="ml-4 text-right">
                    <Link href={`/users/${post.author.id}`}>
                        <span className="text-gray-800 font-semibold hover:underline" aria-label={`View profile of ${post.author.nickname}`}>
                            {post.author.nickname}
                        </span>
                    </Link>
                    <div className="text-gray-500 text-sm">{formattedDate}</div>
                </div>
            </div>

            <Link href={`/posts/${post.id}`}>
                <span className="text-blue-500 hover:underline mt-4 block transition-colors duration-300 hover:text-blue-700" aria-label={`Read more about ${post.title}`}>
                    了解详细
                </span>
            </Link>
        </div>
    );
};

export default PostCard;