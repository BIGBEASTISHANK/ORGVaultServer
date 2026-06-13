"use client";

import { motion } from "framer-motion";
import { useState } from "react";

export default function InitializationLogin({ isRegistered, isAuthenticated }: { isRegistered: any, isAuthenticated: any }) {
    const [adminMacAddress, setAdminMacAddress] = useState("");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState("");

    async function handleFormSubmission(e: React.SubmitEvent<HTMLFormElement>) {
        e.preventDefault();

        setLoading(true);

        // Calling api
        try {
            const API_RESPONSE = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_API_URL}/api/backend/initializeServer`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    adminMacAddress,
                    username,
                    password,
                }),
            });

            if (API_RESPONSE.ok) {
                isAuthenticated(true);
                isRegistered(true);
            }
        } catch (e) {
            setError("Their was error registering the server");
        } finally {
            setLoading(false);
        }
    }

    // Main component
    return (
        <div className="min-h-screen bg-[#0a0f1c] flex items-center justify-center px-6 relative overflow-hidden">
            {/* Background Glow */}
            <div className="absolute w-[600px] h-[600px] bg-[#1793d1]/20 blur-[150px] rounded-full" />

            <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }} className="relative z-10 w-full max-w-md">
                <div className="bg-white/[0.03] backdrop-blur-xl border border-white/10 rounded-3xl p-8 shadow-2xl">
                    {/* Header */}
                    <div className="mb-8">
                        <h1 className="text-3xl font-bold text-white">Initialize Server</h1>

                        <p className="text-white/50 mt-2 text-sm">Configure the first administrator account and authorize the host machine.</p>
                    </div>

                    <form onSubmit={handleFormSubmission} className="space-y-5">
                        {/* MAC Address */}
                        <div>
                            <label className="block text-sm text-white/70 mb-2 select-none">Administrator PC MAC Address</label>

                            <input
                                type="text"
                                placeholder="00:1A:2B:3C:4D:5E"
                                value={adminMacAddress}
                                onChange={(e) => setAdminMacAddress(e.target.value)}
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
