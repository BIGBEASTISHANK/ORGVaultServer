import Image from "next/image";

export default function ProfileBar() {
    return (
        <div className="h-full w-full rounded-xl bg-[#0A0C0E]/30 border border-white/10 backdrop-blur-xl inset-shadow-white/20 inset-shadow-2xs shadow-white/20 shadow-lg">
            {/* Heading Image */}
            <div className="h-[5rem] flex select-none border-b-2 border-white/10">
                <Image src="/ORGVault Heading Geist Mono.png" alt="profile bar heading" width={320} height={160} className="m-auto" />
            </div>
        </div>
    );
}
