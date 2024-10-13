// components/PostCard.js
import Link from 'next/link';

const PostCard = ({ post }) => {
    return (
        <div className="border p-4 rounded shadow hover:shadow-lg transition-shadow duration-300">
            <h3 className="text-xl font-bold">{post.title}</h3>
            <p className="text-gray-700">{post.digest}</p>
            <Link href={`/posts/${post.id}`} legacyBehavior>
                <a className="text-blue-500 hover:underline">Read more</a>
            </Link>
        </div>
    );
};

export default PostCard;
