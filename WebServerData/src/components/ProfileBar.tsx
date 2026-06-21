"use client";
import Image from "next/image";
import { useEffect, useState } from "react";
import { FiLogOut } from "react-icons/fi";

// Interface
interface CurrentAdminDetails {
    name: string;
    macAddress: string;
    username: string;
}

export default function ProfileBar({ logoutFunction, logoutError, loggingOut }: { logoutFunction: any; logoutError: string; loggingOut: boolean }) {
    // Variables
    const [currentAdminDetails, setCurrentAdminDetails] = useState<CurrentAdminDetails>({
        name: "",
        macAddress: "",
        username: "",
    });

    // Fetching current admin details
    useEffect(() => {
        async function fetchCurrentAdminDetails() {
            try {
                const API_RESPONSE = await fetch(`/api/getCurrentAdminDetails`, {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    credentials: "include",
                });

                const API_RESPONSE_DATA = await API_RESPONSE.json();

                if (API_RESPONSE.ok) {
                    setCurrentAdminDetails(API_RESPONSE_DATA.response);
                }
            } catch (err) {
                console.log("Error: ", err);
            }
        }

        fetchCurrentAdminDetails();
    }, []);

    return (
        <div className="h-full w-full rounded-xl bg-[#0A0C0E]/30 border border-white/10 backdrop-blur-xl inset-shadow-white/20 inset-shadow-2xs shadow-white/20 shadow-lg flex flex-col gap-5">
            {/* Heading Image */}
            <div className="h-[5rem] flex select-none border-b-2 border-white/10 px-5">
                <Image src="/ORGVault Heading Geist Mono.png" alt="profile bar heading" width={320} height={160} className="m-auto" draggable={false} loading="eager" />
            </div>

            {/* Current Admin Details */}
            <div className="flex flex-col gap-5 border-b-2 border-white/10 pb-10 px-5">
                {/* Name */}
                <div className="items-center">
                    <h1 className="text-base text-gray-400 font-bold ml-2">Name:</h1>

                    <div className="border border-[#3AB1F5]/30 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/20 inset-shadow-2xs shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                        {!currentAdminDetails.name ? <div className="h-7 w-40 animate-pulse rounded-md bg-white/10" /> : <p className="text-xl">{currentAdminDetails.name}</p>}
                    </div>
                </div>

                {/* MAC Address */}
                <div className="items-center">
                    <h1 className="text-base text-gray-400 font-bold ml-2">MAC Address:</h1>

                    <div className="border border-[#3AB1F5]/30 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/20 inset-shadow-2xs shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                        {!currentAdminDetails.macAddress ? (
                            <div className="h-7 w-52 animate-pulse rounded-md bg-white/10" />
                        ) : (
                            <p className="text-xl">{currentAdminDetails.macAddress}</p>
                        )}
                    </div>
                </div>

                {/* Username */}
                <div className="items-center">
                    <h1 className="text-base text-gray-400 font-bold ml-2">Username:</h1>
                    <div className="border border-[#3AB1F5]/30 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/20 inset-shadow-2xs shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                        {!currentAdminDetails.username ? <div className="h-7 w-32 animate-pulse rounded-md bg-white/10" /> : <p className="text-xl">{currentAdminDetails.username}</p>}
                    </div>
                </div>
            </div>

            {/* Logout Button */}
            <div className="mt-auto">
                <div className="border-t-2 border-white/20 px-5 py-5">
                    <button
                        onClick={logoutFunction}
                        className="w-full h-12 rounded-xl bg-[#FF3333] text-white font-medium shadow-lg shadow-[#FF3333]/25 hover:bg-[#f86868] transition-all select-none cursor-pointer text-xl outline-none"
                    >
                        {loggingOut ? (
                            <div className="flex items-center justify-center gap-3">
                                <div className="h-4 w-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                                Logging out...
                            </div>
                        ) : (
                            <div className="flex items-center justify-center gap-3">
                                Logout <FiLogOut />
                            </div>
                        )}
                    </button>
                </div>
            </div>
        </div>
    );
}
