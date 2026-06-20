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
            <body className="min-h-full flex flex-col">
                <div className="absolute w-[500px] h-[500px] bg-[#3AB1F5] opacity-50 blur-[120px] rounded-full top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 -z-10" />
                {children}
            </body>
        </html>
    );
}
