import jwt from "jsonwebtoken";
import { NextRequest, NextResponse } from "next/server";

export async function POST(req: NextRequest) {
    const cookieHeader = req.headers.get("cookie");
    const KEY_BIN_HASH = process.env.KEY_BIN_HASH;

    if (!cookieHeader) {
        return NextResponse.json({ response: "No cookie header provided" }, { status: 401 });
    }

    try {
        const VERIFY_LOGIN = await fetch(`${req.nextUrl.origin}/api/auth/verifyLogin`, {
            method: "GET",
            headers: {
                Cookie: cookieHeader,
            },
        });

        if (!VERIFY_LOGIN.ok) {
            return NextResponse.json({ response: "Token verification failed" }, { status: 401 });
        }

        const VERIFY_LOGIN_DATA = await VERIFY_LOGIN.json();

        try {
            const CURRENT_ADMIN_DETAILS_API = await fetch(`${process.env.BACKEND_API_URL}/api/backend/currentAdminDetails`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    macAddress: VERIFY_LOGIN_DATA.response,
                    keyBinHash: KEY_BIN_HASH,
                }),
            });

            if (!CURRENT_ADMIN_DETAILS_API.ok) {
                return NextResponse.json({ response: "Internal server error" }, { status: 500 });
            }

            const CURRENT_ADMIN_DETAILS_API_DATA = await CURRENT_ADMIN_DETAILS_API.json();

            return NextResponse.json({ response: CURRENT_ADMIN_DETAILS_API_DATA.response }, { status: 200 });
        } catch (error) {
            return NextResponse.json({ response: "Internal server error" }, { status: 500 });
        }
    } catch (error) {
        console.log(error);
        return NextResponse.json({ response: "Internal server error" }, { status: 500 });
    }
}
