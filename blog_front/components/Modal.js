import React, { useEffect } from 'react';

const Modal = ({ isOpen, onClose, onLogin }) => {
    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
            <div className="bg-white p-6 rounded-lg shadow-lg text-center">
                <h2 className="text-xl font-semibold">用户未登录</h2>
                <p className="mt-2">您需要登录才能访问该页面。</p>
                <div className="mt-4 flex justify-center">
                    <button
                        onClick={onLogin}
                        className="bg-blue-500 text-white py-2 px-4 rounded-lg hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75"
                    >
                        去登录
                    </button>
                    <button
                        onClick={onClose}
                        className="ml-4 bg-gray-300 text-gray-800 py-2 px-4 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 focus:ring-opacity-75"
                    >
                        继续浏览
                    </button>
                </div>
            </div>
        </div>
    );
};

export default Modal;