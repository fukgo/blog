import { useEffect, useState } from 'react';
import { useRouter } from 'next/router';
import { marked } from 'marked';
import Link from 'next/link';
import { FaUser, FaEnvelope } from 'react-icons/fa'; // Icons for user info

const PostDetail = ({ initialArticle, initialUser }) => {
    const router = useRouter();
    const { id } = router.query;

    const [article, setArticle] = useState(initialArticle);
    const [user, setUser] = useState(initialUser);
    const [loading, setLoading] = useState(!initialArticle || !initialUser);
    const [error, setError] = useState(null);

    useEffect(() => {
        marked.setOptions({
            gfm: true, // Enable GitHub flavored markdown
            breaks: true, // Enable line breaks
        });

        const fetchArticle = async () => {
            if (!id) return;

            try {
                const articleResponse = await fetch(`http://127.0.0.1:8002/articles/detail/${id}`, {
                    credentials: 'include'
                });
                if (!articleResponse.ok) throw new Error('Unable to fetch article information');
                const articleData = await articleResponse.json();
                setArticle(articleData);

                const userResponse = await fetch(`http://127.0.0.1:8002/users/${articleData.user_id}`, {
                    credentials: 'include'
                });
                if (!userResponse.ok) throw new Error('Unable to fetch user information');
                const userData = await userResponse.json();
                setUser(userData);
            } catch (error) {
                setError(error.message);
            } finally {
                setLoading(false);
            }
        };

        if (!initialArticle || !initialUser) {
            fetchArticle();
        }
    }, [id, initialArticle, initialUser]);

    if (loading) return <p>Loading...</p>;
    if (error) return <p className="text-red-500">{error}</p>;

    return (
        <div className="container mx-auto p-6 max-w-4xl">
            <div className="bg-white shadow-md rounded-lg overflow-hidden">
                {/* Article Header with Author Info on the Right */}
                <div className="p-6 border-b flex justify-between items-start">
                    <div>
                        <h1 className="text-4xl font-bold text-gray-800 mb-2">{article.title}</h1>
                        <h3 className="text-gray-600 text-sm mb-4 italic">{article.digest || 'No digest available'}</h3>
                    </div>
                    <div className="ml-6 text-right">
                        <h3 className="text-2xl font-semibold mb-4">Author Information</h3>
                        <div className="flex items-center mb-2">
                            <FaUser className="mr-2 text-gray-600" />
                            <p className="text-gray-800">
                                <strong>Username:</strong> {user.username}
                            </p>
                        </div>
                        <div className="flex items-center">
                            <FaEnvelope className="mr-2 text-gray-600" />
                            <p className="text-gray-800">
                                <strong>Email:</strong> {user.email}
                            </p>
                        </div>
                    </div>
                </div>

                {/* Article Content */}
                <div className="p-6">
                    {article.content ? (
                        <div className="prose prose-lg max-w-none" dangerouslySetInnerHTML={{ __html: marked(article.content) }} />
                    ) : (
                        <p className="text-gray-500">This article has no content.</p>
                    )}
                </div>
            </div>

            {/* Back to Posts Button */}
            <Link href="/" className="mt-6 inline-block text-blue-600 hover:underline text-lg">
                ← Back to Posts
            </Link>
        </div>
    );
};

PostDetail.getInitialProps = async ({ query }) => {
    const { id } = query;

    try {
        const articleResponse = await fetch(`http://127.0.0.1:8002/articles/detail/${id}`, {
            credentials: 'include'
        });
        if (!articleResponse.ok) throw new Error('Unable to fetch article information');
        const articleData = await articleResponse.json();

        const userResponse = await fetch(`http://127.0.0.1:8002/users/${articleData.user_id}`, {
            credentials: 'include'
        });
        if (!userResponse.ok) throw new Error('Unable to fetch user information');
        const userData = await userResponse.json();

        return { initialArticle: articleData, initialUser: userData };
    } catch (error) {
        return { initialArticle: null, initialUser: null };
    }
};

export default PostDetail;