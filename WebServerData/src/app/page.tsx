"use client";

import InitializationLogin from "@/components/InitializationLogin";
import NormalLogin from "@/components/NormalLogin";
import ProfileBar from "@/components/ProfileBar";
import LoadingScreen from "@/utilities/LoadingScreen";
import { useEffect, useState } from "react";

export default function Home() {
    // State
    const [isCheckingInit, setIsCheckingInit] = useState(true);
    const [initCheckError, setInitCheckError] = useState("");

    const [needsInitialization, setNeedsInitialization] = useState(true);
    const [isAuthenticated, setIsAuthenticated] = useState(false);

    const [isCheckingAuth, setIsCheckingAuth] = useState(true);

    const [logoutError, setLogoutError] = useState("");
    const [loggingOut, setLoggingOut] = useState(false);

    // Checking initialization
    useEffect(() => {
        async function checkInitialization() {
            try {
                const API_RESPONSE = await fetch("/api/verifyInitializedStatus");

                await new Promise((r) => setTimeout(r, 3000));

                if (API_RESPONSE.status == 200) {
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
                const API_RESPONSE = await fetch("/api/auth/verifyLogin", {
                    method: "GET",
                    credentials: "include",
                });

                if (API_RESPONSE.ok) {
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

    // Logout
    async function logout() {
        try {
            setLoggingOut(true);

            const API_RESPONSE = await fetch("/api/auth/logout");

            if (API_RESPONSE.ok) {
                setIsAuthenticated(false);
            }
        } catch (err) {
            setLogoutError("Internal server error");
        } finally {
            setLoggingOut(false);
        }
    }

    // Loading State
    if (isCheckingInit || isCheckingAuth) {
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
    return (
        <div className="flex gap-5 p-5 h-screen w-screen">
            <div className="h-full w-[30rem]">
                <ProfileBar logoutFunction={logout} logoutError={logoutError} loggingOut={loggingOut} />
            </div>
            <div className="w-full">hi</div>
        </div>
    );
}
