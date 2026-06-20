import Image from "next/image";
import { useState } from "react";

// Interface
interface CurrentAdminDetails {
    name: string;
    macAddress: string;
    username: string;
}

export default function ProfileBar() {
    // Variables
    const [currentAdminDetails, setCurrentAdminDetails] = useState<CurrentAdminDetails>({
        name: "John Doe",
        macAddress: "00:1A:2B:3C:4D:5E",
        username: "admin",
    });

    return (
        <div className="h-full w-full rounded-xl bg-[#0A0C0E]/30 border border-white/10 backdrop-blur-xl inset-shadow-white/20 inset-shadow-2xs shadow-white/20 shadow-lg flex flex-col gap-5">
            {/* Heading Image */}
            <div className="h-[5rem] flex select-none border-b-2 border-white/10 px-5">
                <Image src="/ORGVault Heading Geist Mono.png" alt="profile bar heading" width={320} height={160} className="m-auto" draggable={false} loading="eager"/>
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
                        {!currentAdminDetails.macAddress ? <div className="h-7 w-52 animate-pulse rounded-md bg-white/10" /> : <p className="text-xl">{currentAdminDetails.macAddress}</p>}
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
        </div>
    );
}
