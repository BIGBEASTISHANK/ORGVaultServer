"use client";

import { useState } from "react";
import Image from "next/image";

export default function Home() {
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState("");

    const handleTestApi = async () => {
        try {
            setLoading(true);
            setError("");

            const res = await fetch("http://localhost:3100/api/test");

            if (!res.ok) {
                throw new Error("Failed to fetch API");
            }

            const data = await res.json();

            setMessage(data.message);
        } catch (err) {
            setError("Unable to connect to the API.");
            console.error(err);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="min-h-screen bg-slate-50">
            <main className="mx-auto flex min-h-screen max-w-6xl flex-col px-6 py-12">
                {/* Header */}
                <header className="mb-16 flex items-center justify-between">
                    <div className="flex items-center gap-3">
                        <Image src="/next.svg" alt="Next.js" width={120} height={28} priority />
                    </div>

                    <span className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-600">
                        Prototype v1.0
                    </span>
                </header>

                {/* Hero Section */}
                <section className="mb-12">
                    <h1 className="mb-4 text-5xl font-bold tracking-tight text-slate-900">API Integration Dashboard</h1>

                    <p className="max-w-2xl text-lg text-slate-600">
                        A modern prototype interface for testing backend connectivity, monitoring responses, and
                        validating API integrations in real time.
                    </p>
                </section>

                {/* Main Grid */}
                <section className="grid gap-6 md:grid-cols-2">
                    {/* API Testing Card */}
                    <div className="rounded-2xl border border-slate-200 bg-white p-8 shadow-sm">
                        <h2 className="mb-2 text-xl font-semibold text-slate-900">API Connectivity Test</h2>

                        <p className="mb-6 text-slate-600">
                            Verify communication with the backend service and display the returned response.
                        </p>

                        <button
                            onClick={handleTestApi}
                            disabled={loading}
                            className="rounded-xl bg-slate-900 px-5 py-3 font-medium text-white transition hover:bg-slate-800 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {loading ? "Testing..." : "Run API Test"}
                        </button>
                    </div>

                    {/* Response Card */}
                    <div className="rounded-2xl border border-slate-200 bg-white p-8 shadow-sm">
                        <h2 className="mb-4 text-xl font-semibold text-slate-900">Response</h2>

                        {!message && !error && (
                            <div className="rounded-xl bg-slate-50 p-4 text-slate-500">No response received yet.</div>
                        )}

                        {message && (
                            <div className="rounded-xl border border-green-200 bg-green-50 p-4">
                                <p className="text-sm font-medium text-green-700">Success</p>
                                <p className="mt-2 text-green-900">{message}</p>
                            </div>
                        )}

                        {error && (
                            <div className="rounded-xl border border-red-200 bg-red-50 p-4">
                                <p className="text-sm font-medium text-red-700">Error</p>
                                <p className="mt-2 text-red-900">{error}</p>
                            </div>
                        )}
                    </div>
                </section>

                {/* Footer */}
                <footer className="mt-auto pt-16">
                    <div className="rounded-2xl border border-slate-200 bg-white p-6">
                        <h3 className="mb-2 font-semibold text-slate-900">System Status</h3>
                        <p className="text-slate-600">
                            Frontend prototype is operational and ready for API integration testing.
                        </p>
                    </div>
                </footer>
            </main>
        </div>
    );
}
