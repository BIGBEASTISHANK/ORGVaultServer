"use client";

import InitializationLogin from "@/components/InitializationLogin";
import NormalLogin from "@/components/NormalLogin";
import LoadingScreen from "@/utilities/LoadingScreen";
import { useEffect, useState } from "react";

export default function Home() {
    // State
    const [isCheckingInit, setIsCheckingInit] = useState(true);
    const [initCheckError, setInitCheckError] = useState("");

    const [needsInitialization, setNeedsInitialization] = useState(true);
    const [isAuthenticated, setIsAuthenticated] = useState(false);

    const [isCheckingAuth, setIsCheckingAuth] = useState(true);

    // Checking initialization
    useEffect(() => {
        async function checkInitialization() {
            try {
                const API_RESPONSE = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_API_URL}/api/backend/initializedStatus`);

                await new Promise((r) => setTimeout(r, 3000));

                if (API_RESPONSE.ok) {
                    setNeedsInitialization(false);
                }
            } catch (err) {
                setInitCheckError("Internal server error");
            } finally {
                setIsCheckingInit(false);
            }
        }

        checkInitialization();
    }, []);

    // Checking authentication
    useEffect(() => {
        async function checkAuthSession() {
            if (needsInitialization) {
                setIsCheckingAuth(false);
                return;
            }

            try {
                // const API_RESPONSE = await fetch("/api/auth/verify", {
                //     method: "GET",
                //     credentials: "include",
                // });

                // if (API_RESPONSE.ok) {
                if(false) {
                    setIsAuthenticated(true);
                }
            } catch (err) {
                setIsAuthenticated(false);
            } finally {
                setIsCheckingAuth(false);
            }
        }

        checkAuthSession();
    }, [needsInitialization]);

    // Loading State
    if (isCheckingInit) {
        return <LoadingScreen error={initCheckError} />;
    }

    // Initializing form
    if (needsInitialization && !isAuthenticated) {
        return <InitializationLogin isRegistered={setNeedsInitialization} isAuthenticated={setIsAuthenticated} />;
    }

    // Authenticated
    if (!isAuthenticated && !needsInitialization) {
        return <NormalLogin isAuthenticated={setIsAuthenticated} />;
    }

    //  Normal Page
    return <div>App Home</div>;
}
