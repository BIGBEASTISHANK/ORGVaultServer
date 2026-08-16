"use client";
import Image from "next/image";
import { useEffect, useState } from "react";
import { FiLogOut } from "react-icons/fi";
import { GrUpdate } from "react-icons/gr";
import { ImCancelCircle } from "react-icons/im";

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
    const [passwordUpdateError, setPasswordUpdateError] = useState("");
    const [passwordUpdateSuccess, setPasswordUpdateSuccess] = useState("");
    const [updatingPasswordLoading, setUpdatingPasswordLoading] = useState(false);

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

    // Updating password handler function
    async function updatePasswordHandler() {
        setPasswordUpdateError("");
        setPasswordUpdateSuccess("");

        if (!passwordForm.currentPassword || !passwordForm.newPassword || !passwordForm.confirmPassword) {
            return setPasswordUpdateError("All fields are required.");
        }

        try {
            setUpdatingPasswordLoading(true);

            const UPDATE_PASSWORD_API = await fetch("/api/auth/updateAdminPassword", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify({
                    currentPassword: passwordForm.currentPassword,
                    newPassword: passwordForm.newPassword,
                    confirmPassword: passwordForm.confirmPassword,
                }),
            });

            const data = await UPDATE_PASSWORD_API.json();

            if (!UPDATE_PASSWORD_API.ok) {
                setPasswordUpdateError(data?.response || "Failed to update password.");

                return;
            }

            setPasswordUpdateSuccess(data?.response || "Password updated successfully.");

            setPasswordForm({
                currentPassword: "",
                newPassword: "",
                confirmPassword: "",
            });

            setUpdatingPassword(false);

            setTimeout(() => {
                setPasswordUpdateSuccess("");
            }, 2000);
        } catch (error) {
            setPasswordUpdateError("Something went wrong. Please try again.");
        } finally {
            setUpdatingPasswordLoading(false);
        }
    }

    return (
        <div className="h-full w-full rounded-xl bg-[#0A0C0E]/30 border border-white/10 backdrop-blur-xl inset-shadow-white/30 inset-shadow-2xs shadow-white/20 shadow-lg flex flex-col gap-5">
            {/* Heading Image */}
            <div className="py-2 flex select-none border-b-2 border-white/10 px-5">
                <Image src="/ORGVault Heading Geist Mono.png" alt="profile bar heading" width={320} height={160} className="m-auto" draggable={false} loading="eager" />
            </div>

            {/* Current Admin Details */}
            <div className="flex flex-col gap-5 border-b-2 border-white/10 pb-7 px-5 bg-red-">
                {!updatingPassword ? (
                    <>
                        {/* Name */}
                        <div className="items-center">
                            <h1 className="text-sm text-gray-400 font-bold ml-2">Name:</h1>

                            <div className="border border-white/10 shadow-sm bg-white/[0.02] shadow-white/30 rounded-xl backdrop-blur-xl px-2 py-1">
                                {!currentAdminDetails.name ? <div className="h-7 w-full animate-pulse rounded-md bg-white/10" /> : <p className="text-lg">{currentAdminDetails.name}</p>}
                            </div>
                        </div>

                        {/* MAC Address */}
                        <div className="items-center">
                            <h1 className="text-sm text-gray-400 font-bold ml-2">MAC Address:</h1>

                            <div className="border border-white/10 shadow-sm bg-white/[0.02] shadow-white/30 rounded-xl backdrop-blur-xl px-2 py-1">
                                {!currentAdminDetails.macAddress ? (
                                    <div className="h-7 w-full animate-pulse rounded-md bg-white/10" />
                                ) : (
                                    <p className="text-lg">{currentAdminDetails.macAddress}</p>
                                )}
                            </div>
                        </div>

                        {/* Username */}
                        <div className="items-center">
                            <h1 className="text-sm text-gray-400 font-bold ml-2">Username:</h1>

                            <div className="border border-white/10 shadow-sm bg-white/[0.02] shadow-white/30 rounded-xl backdrop-blur-xl px-2 py-1">
                                {!currentAdminDetails.username ? (
                                    <div className="h-7 w-full animate-pulse rounded-md bg-white/10" />
                                ) : (
                                    <p className="text-lg">{currentAdminDetails.username}</p>
                                )}
                            </div>
                        </div>

                        {/* Feedback */}
                        {passwordUpdateSuccess && <div className="rounded-xl border border-green-500/40 bg-green-500/10 px-3 py-2 text-green-400 text-sm">{passwordUpdateSuccess}</div>}

                        {/* Update Password Button */}
                        <div className="items-center">
                            <button
                                onClick={() => setUpdatingPassword(true)}
                                className="rounded-xl bg-[#21A3EE] hover:bg-[#21A3EE]/80 shadow-sm shadow-[#21A3EE]/50 inset-shadow-xs inset-shadow-white/80 flex items-center gap-2 px-3 py-1 text-white font-medium transition-all select-none cursor-pointer text-lg outline-none"
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

                            <div className="border border-white/10 shadow-sm bg-white/[0.02] shadow-white/30 focus-within:border-[#21A3EE] focus-within:shadow-[#21A3EE]/50 focus-within:shadow-md rounded-xl backdrop-blur-xl px-2 py-1">
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

                            <div className="border border-white/10 shadow-sm bg-white/[0.02] shadow-white/30 focus-within:border-[#21A3EE] focus-within:shadow-[#21A3EE]/50 focus-within:shadow-md rounded-xl backdrop-blur-xl px-2 py-1">
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

                            <div className="border border-white/10 shadow-sm bg-white/[0.02] shadow-white/30 focus-within:border-[#21A3EE] focus-within:shadow-[#21A3EE]/50 focus-within:shadow-md rounded-xl backdrop-blur-xl px-2 py-1">
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

                        {/* Feedback */}
                        {passwordUpdateError && <div className="rounded-xl border border-red-500/40 bg-red-500/10 px-3 py-2 text-red-400 text-sm">{passwordUpdateError}</div>}
                        {passwordUpdateSuccess && <div className="rounded-xl border border-green-500/40 bg-green-500/10 px-3 py-2 text-green-400 text-sm">{passwordUpdateSuccess}</div>}

                        {/* Buttons */}
                        <div className="flex gap-3">
                            <button
                                onClick={updatePasswordHandler}
                                className="flex-1 flex items-center justify-center bg-[#21A3EE] hover:bg-[#21A3EE]/80 shadow-sm shadow-[#21A3EE]/50 inset-shadow-xs inset-shadow-white/80 gap-2 rounded-full px-3 py-1 text-white font-medium transition-all select-none cursor-pointer text-lg outline-none"
                            >
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

                                    setPasswordUpdateError("");
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
            <div className="h-full bg-red-500/20"></div>

            {/* Logout Button */}
            <div className="mt-auto">
                <div className="border-t-2 border-white/20 px-5 py-5">
                    <button
                        onClick={logoutFunction}
                        className="w-full py-3 rounded-2xl text-white font-medium shadow-sm shadow-[#21A3EE]/50 inset-shadow-xs inset-shadow-white/80 bg-[#21A3EE] hover:bg-[#21A3EE]/75 transition-all select-none cursor-pointer text-xl outline-none"
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
