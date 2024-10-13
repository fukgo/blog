import Head from 'next/head';
import Link from 'next/link';
import { useEffect, useState } from 'react';
import axios from 'axios';

export default function Layout({ children }) {
    const [tags, setTags] = useState([]); // State to store tags
    const [users, setUsers] = useState([]); // State to store users
    const [loadingTags, setLoadingTags] = useState(true); // Loading state for tags
    const [loadingUsers, setLoadingUsers] = useState(true); // Loading state for users
    const [errorTags, setErrorTags] = useState(null); // Error state for tags
    const [errorUsers, setErrorUsers] = useState(null); // Error state for users

    // Function to fetch tags from the API
    const fetchTags = async () => {
        try {
            const token = sessionStorage.getItem('authToken'); // 从 sessionStorage 获取 token

            const response = await axios.get('http://127.0.0.1:8002/tags/all', {
                headers: {
                    'Authorization': `Bearer ${token}`, // 在请求头中添加 token
                },
                withCredentials: true
            });
            setTags(response.data); // Set the fetched tags
        } catch (err) {
            setErrorTags(err); // Handle error
        } finally {
            setLoadingTags(false); // End loading
        }
    };

    // Function to fetch users from the API
    const fetchUsers = async () => {
        try {
            const token = sessionStorage.getItem('authToken'); // 从 sessionStorage 获取 token

            const response = await axios.get('http://127.0.0.1:8002/users', {
                headers: {
                    'Authorization': `Bearer ${token}`, // 在请求头中添加 token
                },
                withCredentials: true
            });
            setUsers(response.data); // Set the fetched users
        } catch (err) {
            setErrorUsers(err); // Handle error
        } finally {
            setLoadingUsers(false); // End loading
        }
    };

    // Fetch tags and users on component mount
    useEffect(() => {
        fetchTags();
        fetchUsers();
    }, []);

    // Handle loading states
    if (loadingTags || loadingUsers) {
        return <div>Loading...</div>; // Display loading state
    }

    // Handle error states
    if (errorTags) {
        return <div>Error fetching tags: {errorTags.message}</div>; // Display error message
    }

    if (errorUsers) {
        return <div>Error fetching users: {errorUsers.message}</div>; // Display error message
    }

    return (
        <div className="min-h-screen flex flex-col">
            <Head>
                <title>My Blog</title>
                <meta name="description" content="A personal blog built with Next.js" />
            </Head>

            {/* Header */}
            <header className="bg-white shadow-sm">
                <div className="container mx-auto px-4 py-4">
                    <nav className="flex justify-between items-center">
                        <Link href="/" className="text-2xl font-bold text-gray-900">My Blog</Link>
                        <div className="space-x-4">
                            <Link href="/" className="text-gray-700 hover:text-gray-900">About</Link>
                            <Link href="/" className="text-gray-700 hover:text-gray-900">Blog</Link>
                            <Link href="/" className="text-gray-700 hover:text-gray-900">Contact</Link>

                            {/* Add new article/tag buttons */}
                            <Link href="/posts/new" className="bg-blue-500 text-white px-3 py-1 rounded hover:bg-blue-600">New Article</Link>
                            <Link href="/tags/new" className="bg-green-500 text-white px-3 py-1 rounded hover:bg-green-600">New Tag</Link>
                        </div>
                    </nav>
                </div>
            </header>

            {/* Main Content */}
            <div className="flex flex-grow container mx-auto px-4 py-6">
                {/* Sidebar */}
                <aside className="w-64 bg-gray-50 border-r hidden lg:block">
                    <div className="p-6">
                        <h2 className="text-lg font-bold text-gray-900 mb-4">Tags</h2>
                        <ul className="space-y-2">
                            {tags.map((tag) => (
                                <li key={tag.id}>
                                    <Link href={`/tags/${tag.id}`} className="text-gray-600 hover:text-gray-900">
                                        {tag.tag}
                                    </Link>
                                </li>
                            ))}
                        </ul>

                        {/* Authors Section */}
                        <h2 className="text-lg font-bold text-gray-900 mt-6 mb-4">Authors</h2>
                        <ul className="space-y-1">
                            {users.map((user) => (
                                <Link key={user.id} href={`/users/${user.id}`} className="text-gray-600">
                                    {user.username}
                                </Link>
                            ))}
                        </ul>
                    </div>
                </aside>

                {/* Main Content Area */}
                <main className="flex-grow bg-white p-6 shadow-sm">
                    {children}
                </main>
            </div>

            {/* Footer */}
            <footer className="bg-white border-t py-4">
                <div className="container mx-auto text-center text-gray-500">
                    © 2024 My Blog - All Rights Reserved
                </div>
            </footer>
        </div>
    );
}
