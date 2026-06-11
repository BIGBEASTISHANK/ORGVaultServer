import type { Metadata } from "next";
import { Geist_Mono } from "next/font/google";
import "@/styles/tailwindimport.css";
import "@/styles/global.scss";

const geistMono = Geist_Mono({
    variable: "--font-geist-mono",
    subsets: ["latin"],
});

export const metadata: Metadata = {
    title: "ORGVault Server CP",
    description: "ORGVault server control panel",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en" className={`${geistMono.className} h-full antialiased`}>
            <body className="min-h-full flex flex-col">{children}</body>
        </html>
    );
}
