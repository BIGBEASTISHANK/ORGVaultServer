"use client";

import InitializationLogin from "@/components/InitializationLogin";
import NormalLogin from "@/components/NormalLogin";
import LoadingScreen from "@/utilities/LoadingScreen";
import { useEffect, useState } from "react";

export default function Home() {
    const [isInitialized, setIsInitialized] = useState(false);
    const [verifyingHome, setVerifyingHome] = useState(true);

    useEffect(() => {
        // Function to check if initialized
        async function checkInitializedStatus() {
            try {
                // Calling api
                const apiResponse = await fetch(
                    `${process.env.NEXT_PUBLIC_API_URL}/api/initializedStatus`,
                    {
                        method: "GET",
                    },
                );

                // Wait for 3 seconds
                await new Promise((resolve) => setTimeout(resolve, 3000));

                if (apiResponse.ok) {
                    setVerifyingHome(false);
                    setIsInitialized(true);
                }
            } catch (e) {
                console.log(e);
            } finally {
                setVerifyingHome(false);
            }
        }

        // Calling API
        checkInitializedStatus();
    }, []);

    // Main diaplay
    return verifyingHome ? (
        <LoadingScreen />
    ) : isInitialized ? (
        <NormalLogin />
    ) : (
        <InitializationLogin />
    );
}
