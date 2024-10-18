import { useEffect, useState } from 'react';
import axios from 'axios';
import {getAllTagsUrl} from "@/api_list";

const TagsList = () => {
    const [tags, setTags] = useState([]); // State to store tags
    const [newTag, setNewTag] = useState(''); // State to store new tag input
    const [loading, setLoading] = useState(true); // Loading state
    const [error, setError] = useState(null); // Error state

    // Function to fetch tags from the API
    const fetchTags = async () => {
        try {
            const response = await axios.get(getAllTagsUrl(), {
                withCredentials: true
            });
            setTags(response.data); // Set the fetched tags
        } catch (err) {
            setError(err); // Handle error
        } finally {
            setLoading(false); // End loading
        }
    };

    // Function to delete a tag
    const deleteTag = async (tagId) => {
        try {
            await axios.delete(deleteTagUrl(tagId), {
                withCredentials: true
            });
            setTags(tags.filter(tag => tag.id !== tagId)); // Remove the deleted tag from the state
        } catch (err) {
            setError(err); // Handle error
        }
    };

    // Function to add a new tag
    const addTag = async (e) => {
        e.preventDefault();
        try {
            await axios.get(createTagUrl(newTag), {
                withCredentials: true
            });
            fetchTags(); // Refresh the tags list
            setNewTag(''); // Clear the input field
        } catch (err) {
            setError(err); // Handle error
        }
    };

    // Fetch tags when the component mounts
    useEffect(() => {
        fetchTags();
    }, []);

    return (
        <div className="container mx-auto p-6 max-w-4xl">
            <h1 className="text-4xl font-bold mb-4">标签列表</h1>

            {/* Display loading state */}
            {loading && <div>Loading...</div>}

            {/* Display error state */}
            {error && <div className="text-red-500">错误: {error.message}</div>}

            {/* Form to add a new tag */}
            <form onSubmit={addTag} className="mb-4">
                <input
                    type="text"
                    className="w-full p-2 border mb-4"
                    placeholder="New Tag"
                    value={newTag}
                    onChange={(e) => setNewTag(e.target.value)}
                    required
                />
                <button className="bg-blue-500 text-white px-4 py-2 rounded">增加标签</button>
            </form>

            {/* Display the list of tags */}
            <ul className="space-y-2">
                {tags.map(tag => (
                    <li key={tag.id} className="flex justify-between items-center p-2 border rounded">
                        <span>{tag.tag}</span>
                        <button
                            onClick={() => deleteTag(tag.id)}
                            className="bg-red-500 text-white px-2 py-1 rounded"
                        >
                            删除
                        </button>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default TagsList;