import jwt from "jsonwebtoken";
import { NextRequest, NextResponse } from "next/server";

export async function GET(req: NextRequest) {
    const TOKEN = req.cookies.get("token")?.value;
    const KEY_BIN_HASH = process.env.KEY_BIN_HASH;

    if (!KEY_BIN_HASH) {
        return NextResponse.json({ response: "Internal server error" }, { status: 500 });
    }

    if (!TOKEN) {
        return NextResponse.json({ response: "No token provided" }, { status: 401 });
    }

    try {
        const DECODED = jwt.verify(TOKEN, KEY_BIN_HASH) as { MAC_ADDRESS: string };

        return NextResponse.json(
            {
                response: DECODED.MAC_ADDRESS,
            },
            { status: 200 },
        );
    } catch (error) {
        return NextResponse.json({ response: "Invalid or expired token" }, { status: 401 });
    }
}
