//ProtectedComponent.js
import React, { useState, useEffect } from 'react';
import useAuth from './useAuth';
import Modal from './Modal';
import {authTokenUrl, loginUrl} from "@/api_list";

const ParentComponent = ({ children }) => {
    const { isAuthenticated, loading } = useAuth();
    const [isModalOpen, setIsModalOpen] = useState(false);

    const handleClose = () => {
        setIsModalOpen(false);
        window.location.href = '/';
    };

    const handleLogin = () => {
        // const redirectUrl = encodeURIComponent(localStorage.getItem('redirect_url'));
        const currentDomain = window.location.origin;
        window.location.href = loginUrl();
    };

    useEffect(() => {
        if (!loading && !isAuthenticated) {
            setIsModalOpen(true);
        }
    }, [loading, isAuthenticated]);

    if (loading) {
        return <div>Loading...</div>;
    }

    return (
        <div>
            {isModalOpen && <Modal isOpen={isModalOpen} onClose={handleClose} onLogin={handleLogin} />}
            {isAuthenticated && children}
        </div>
    );
};

export default ParentComponent;