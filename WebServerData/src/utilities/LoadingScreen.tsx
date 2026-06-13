"use client";

import { motion } from "framer-motion";

export default function LoadingScreen({error}: {error: string}) {
    const hasError = Boolean(error);

    return (
        <div className="h-screen w-full flex items-center justify-center bg-[#0b0f17] relative overflow-hidden">
            {/* subtle glow background */}
            <div className="absolute w-[500px] h-[500px] bg-[#0f5fff] opacity-20 blur-[120px] rounded-full top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />

            <motion.div
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.5, ease: "easeOut" }}
                className="relative flex flex-col items-center gap-6 px-10 py-10 rounded-2xl 
                   bg-white/5 border border-white/10 backdrop-blur-xl shadow-2xl"
            >
                {/* Spinner or Error */}
                {!hasError ? (
                    <div className="relative w-16 h-16">
                        {/* outer ring */}
                        <div className="absolute inset-0 rounded-full border border-white/10" />

                        {/* animated ring */}
                        <motion.div
                            className="absolute inset-0 rounded-full border-2 border-t-[#0f5fff] border-r-transparent border-b-transparent border-l-transparent"
                            animate={{ rotate: 360 }}
                            transition={{
                                repeat: Infinity,
                                duration: 0.8,
                                ease: "linear",
                            }}
                        />

                        {/* inner glow dot */}
                        <div className="absolute inset-0 flex items-center justify-center">
                            <div className="w-2 h-2 rounded-full bg-[#0f5fff] shadow-[0_0_15px_#0f5fff]" />
                        </div>
                    </div>
                ) : (
                    <div className="text-center">
                        <div className="w-10 h-10 mx-auto mb-3 rounded-full bg-red-500/20 flex items-center justify-center">
                            <span className="text-red-400 text-lg">!</span>
                        </div>

                        <p className="text-red-400 text-sm font-medium">Something went wrong</p>
                        <p className="text-white/50 text-xs mt-1 max-w-[220px]">{error}</p>
                    </div>
                )}

                {/* Text (hide or adjust on error) */}
                {!hasError && (
                    <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="text-center">
                        <p className="text-white text-sm tracking-wide">Taking permission from server</p>
                        <p className="text-white/40 text-xs mt-1">verifying credentials securely...</p>
                    </motion.div>
                )}
            </motion.div>
        </div>
    );
}
