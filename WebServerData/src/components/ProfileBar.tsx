"use client";
import Image from "next/image";
import { useEffect, useState } from "react";
import { FiLogOut } from "react-icons/fi";
import { GrUpdate } from "react-icons/gr";
import { ImCancelCircle } from "react-icons/im";
import { MdUpdate } from "react-icons/md";

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

    const [updatingPassword, setUpdatingPassword] = useState(false);

    const [passwordForm, setPasswordForm] = useState({
        currentPassword: "",
        newPassword: "",
        confirmPassword: "",
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
        <div className="h-full w-full rounded-xl bg-[#0A0C0E]/30 border border-white/10 backdrop-blur-xl inset-shadow-white/30 inset-shadow-2xs shadow-white/20 shadow-lg flex flex-col gap-5">
            {/* Heading Image */}
            <div className="py-2 flex select-none border-b-2 border-white/10 px-5">
                <Image src="/ORGVault Heading Geist Mono.png" alt="profile bar heading" width={320} height={160} className="m-auto" draggable={false} loading="eager" />
            </div>

            {/* Current Admin Details / Update Password */}
            <div className="flex flex-col gap-5 border-b-2 border-white/10 pb-7 px-5">
                {!updatingPassword ? (
                    <>
                        {/* Name */}
                        <div className="items-center">
                            <h1 className="text-sm text-gray-400 font-bold ml-2">Name:</h1>

                            <div className="border border-[#3AB1F5]/80 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/50 inset-shadow-sm shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                                {!currentAdminDetails.name ? <div className="h-7 w-40 animate-pulse rounded-md bg-white/10" /> : <p className="text-lg">{currentAdminDetails.name}</p>}
                            </div>
                        </div>

                        {/* MAC Address */}
                        <div className="items-center">
                            <h1 className="text-sm text-gray-400 font-bold ml-2">MAC Address:</h1>

                            <div className="border border-[#3AB1F5]/80 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/50 inset-shadow-sm shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                                {!currentAdminDetails.macAddress ? (
                                    <div className="h-7 w-52 animate-pulse rounded-md bg-white/10" />
                                ) : (
                                    <p className="text-lg">{currentAdminDetails.macAddress}</p>
                                )}
                            </div>
                        </div>

                        {/* Username */}
                        <div className="items-center">
                            <h1 className="text-sm text-gray-400 font-bold ml-2">Username:</h1>

                            <div className="border border-[#3AB1F5]/80 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/50 inset-shadow-sm shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                                {!currentAdminDetails.username ? (
                                    <div className="h-7 w-32 animate-pulse rounded-md bg-white/10" />
                                ) : (
                                    <p className="text-lg">{currentAdminDetails.username}</p>
                                )}
                            </div>
                        </div>

                        {/* Update Password Button */}
                        <div className="items-center">
                            <button
                                onClick={() => setUpdatingPassword(true)}
                                className="rounded-full flex items-center gap-2 px-3 py-1 text-white font-medium shadow-lg shadow-[#00FF00]/25 inset-shadow-sm inset-shadow-[#00FF00] border border-[#32f332] hover:bg-[#00FF00]/75 transition-all select-none cursor-pointer text-lg outline-none"
                            >
                                Update Password <GrUpdate className="text-sm" />
                            </button>
                        </div>
                    </>
                ) : (
                    <>
                        {/* Current Password */}
                        <div>
                            <h1 className="text-sm text-gray-400 font-bold ml-2">Current Password:</h1>

                            <div className="border border-[#3AB1F5]/80 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/50 inset-shadow-sm shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                                <input
                                    type="password"
                                    placeholder="Enter current password"
                                    value={passwordForm.currentPassword}
                                    onChange={(e) =>
                                        setPasswordForm((prev) => ({
                                            ...prev,
                                            currentPassword: e.target.value,
                                        }))
                                    }
                                    className="w-full bg-transparent outline-none text-lg"
                                />
                            </div>
                        </div>

                        {/* New Password */}
                        <div>
                            <h1 className="text-sm text-gray-400 font-bold ml-2">New Password:</h1>

                            <div className="border border-[#3AB1F5]/80 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/50 inset-shadow-sm shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                                <input
                                    type="password"
                                    placeholder="Enter new password"
                                    value={passwordForm.newPassword}
                                    onChange={(e) =>
                                        setPasswordForm((prev) => ({
                                            ...prev,
                                            newPassword: e.target.value,
                                        }))
                                    }
                                    className="w-full bg-transparent outline-none text-lg"
                                />
                            </div>
                        </div>

                        {/* Confirm Password */}
                        <div>
                            <h1 className="text-sm text-gray-400 font-bold ml-2">Confirm Password:</h1>

                            <div className="border border-[#3AB1F5]/80 rounded-xl backdrop-blur-xl inset-shadow-[#3AB1F5]/50 inset-shadow-sm shadow-[#3AB1F5]/20 shadow-lg px-2 py-1">
                                <input
                                    type="password"
                                    placeholder="Confirm new password"
                                    value={passwordForm.confirmPassword}
                                    onChange={(e) =>
                                        setPasswordForm((prev) => ({
                                            ...prev,
                                            confirmPassword: e.target.value,
                                        }))
                                    }
                                    className="w-full bg-transparent outline-none text-lg"
                                />
                            </div>
                        </div>

                        {/* Buttons */}
                        <div className="flex gap-3">
                            <button className="flex-1 flex items-center justify-center gap-2 rounded-full px-3 py-1 text-white font-medium shadow-lg shadow-[#00FF00]/25 inset-shadow-sm inset-shadow-[#00FF00] border border-[#32f332] hover:bg-[#00FF00]/75 transition-all select-none cursor-pointer text-lg outline-none">
                                Update <GrUpdate className="text-sm" />
                            </button>

                            <button
                                onClick={() => {
                                    setUpdatingPassword(false);

                                    setPasswordForm({
                                        currentPassword: "",
                                        newPassword: "",
                                        confirmPassword: "",
                                    });
                                }}
                                className="flex-1 flex items-center justify-center gap-2 rounded-full px-3 py-1 text-white font-medium shadow-lg shadow-[#FF3333]/25 inset-shadow-sm inset-shadow-[#FF3333] border border-[#f95252] hover:bg-[#FF3333]/75 transition-all select-none cursor-pointer text-lg outline-none"
                            >
                                Cancel <ImCancelCircle />
                            </button>
                        </div>
                    </>
                )}
            </div>

            {/* Other admins */}
            <div className="h-full"></div>

            {/* Logout Button */}
            <div className="mt-auto">
                <div className="border-t-2 border-white/20 px-5 py-5">
                    <button
                        onClick={logoutFunction}
                        className="w-full py-3 rounded-2xl text-white font-medium shadow-xl shadow-[#FF3333]/25 inset-shadow-sm inset-shadow-[#FF3333] border border-[#f95252] hover:bg-[#FF3333]/75 transition-all select-none cursor-pointer text-xl outline-none"
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
