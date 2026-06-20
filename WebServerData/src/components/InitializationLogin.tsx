"use client";

import { motion } from "framer-motion";
import { useState } from "react";

export default function InitializationLogin({ isRegistered, isAuthenticated }: { isRegistered: any; isAuthenticated: any }) {
    const [adminName, setAdminName] = useState<string>("");
    const [adminMacAddress, setAdminMacAddress] = useState<string>("");
    const [username, setUsername] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string>("");

    async function handleFormSubmission(e: React.SubmitEvent<HTMLFormElement>) {
        e.preventDefault();

        setLoading(true);

        // Calling api
        try {
            // Validating format
            const ADMIN_MAC_FORMAT = /^([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}$/;

            if (!adminMacAddress || !ADMIN_MAC_FORMAT.test(adminMacAddress)) {
                setError("Invalid mac address format");
                return;
            }

            if (!username || /\s/.test(username)) {
                setError("Username cannot contain spaces");
                return;
            }

            if (!password) {
                setError("Password cannot be empty");
                return;
            }

            // Removing error
            setError("");

            // Registering
            const API_RESPONSE = await fetch("/api/auth/register", {
                method: "POST",
                body: JSON.stringify({
                    name: adminName,
                    adminMacAddress,
                    username,
                    password,
                }),
            });

            if (!API_RESPONSE.ok) {
                const API_RESPONSE_DATA = await API_RESPONSE.json();
                setError(API_RESPONSE_DATA.response);
                return;
            }

            isAuthenticated(true);
            isRegistered(true);

            const RESPONSE_DATA = await API_RESPONSE.json();
            setError(RESPONSE_DATA.response);
        } catch (e) {
            setError("Their was error registering the server");
        } finally {
            setLoading(false);
        }
    }

    // Main component
    return (
        <div className="min-h-screen flex items-center justify-center px-6 relative overflow-hidden">
            <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }} className="relative z-10 w-full max-w-md">
                <div className="bg-black/[0.05] backdrop-blur-xl inset-shadow-white/20 inset-shadow-2xs shadow-white/20 shadow-lg rounded-3xl p-8">
                    {/* Header */}
                    <div className="mb-8">
                        <h1 className="text-3xl font-bold text-white">Initialize Server</h1>

                        <p className="text-white/50 mt-2 text-sm">Configure the first administrator account and authorize the host machine.</p>
                    </div>

                    <form onSubmit={handleFormSubmission} className="space-y-5">
                        {/* Admin name */}
                        <div>
                            <label className="block text-sm text-white/70 mb-2 select-none">Administrator Name</label>

                            <input
                                type="text"
                                placeholder="John Doe"
                                value={adminName}
                                onChange={(e) => setAdminName(e.target.value)}
                                className="w-full rounded-xl bg-white/[0.04] border border-white/10 px-4 py-3 text-white placeholder:text-white/30 outline-none focus:border-[#1793d1] focus:ring-2 focus:ring-[#1793d1]/20 transition placeholder:select-none"
                                required
                            />
                        </div>

                        {/* MAC Address */}
                        <div>
                            <label className="block text-sm text-white/70 mb-2 select-none">Administrator PC MAC Address</label>

                            <input
                                type="text"
                                placeholder="00:1A:2B:3C:4D:5E"
                                value={adminMacAddress}
                                onChange={(e) => setAdminMacAddress(e.target.value.toUpperCase())}
                                className="w-full rounded-xl bg-white/[0.04] border border-white/10 px-4 py-3 text-white placeholder:text-white/30 outline-none focus:border-[#1793d1] focus:ring-2 focus:ring-[#1793d1]/20 transition placeholder:select-none"
                                required
                            />
                        </div>

                        {/* Username */}
                        <div>
                            <label className="block text-sm text-white/70 mb-2 select-none">Administrator Username</label>

                            <input
                                type="text"
                                placeholder="admin"
                                value={username}
                                onChange={(e) => setUsername(e.target.value)}
                                className="w-full rounded-xl bg-white/[0.04] border border-white/10 px-4 py-3 text-white placeholder:text-white/30 outline-none focus:border-[#1793d1] focus:ring-2 focus:ring-[#1793d1]/20 transition placeholder:select-none"
                                required
                            />
                        </div>

                        {/* Password */}
                        <div>
                            <label className="block text-sm text-white/70 mb-2 select-none">Create Password</label>

                            <input
                                type="password"
                                placeholder="••••••••••••"
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                                className="w-full rounded-xl bg-white/[0.04] border border-white/10 px-4 py-3 text-white placeholder:text-white/30 outline-none focus:border-[#1793d1] focus:ring-2 focus:ring-[#1793d1]/20 transition placeholder:select-none"
                                required
                            />
                        </div>

                        {/* Error */}
                        {error && <div className="rounded-xl border border-red-500/20 bg-red-500/10 px-4 py-3 text-sm text-red-400">{error}</div>}

                        {/* Submit */}
                        <motion.button
                            whileHover={{ scale: 1.02 }}
                            whileTap={{ scale: 0.98 }}
                            disabled={loading}
                            type="submit"
                            className="w-full h-12 rounded-xl bg-[#1793d1] text-white font-medium shadow-lg shadow-[#1793d1]/25 hover:bg-[#1ca4ea] transition disabled:opacity-60 disabled:cursor-not-allowed select-none"
                        >
                            {loading ? (
                                <div className="flex items-center justify-center gap-3">
                                    <div className="h-4 w-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                                    Initializing...
                                </div>
                            ) : (
                                "Initialize Server"
                            )}
                        </motion.button>
                    </form>
                </div>
            </motion.div>
        </div>
    );
}
