import { NextRequest, NextResponse } from "next/server";

export async function POST(req: NextRequest) {
    const cookieHeader = req.headers.get("cookie");

    const REQ_BODY = await req.json();
    let macAddress = "";
    let currentPassword = REQ_BODY.currentPassword;
    let newPassword = REQ_BODY.newPassword;
    let confirmPassword = REQ_BODY.confirmPassword;

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
        macAddress = VERIFY_LOGIN_DATA.response;

        // Formatting check
        const MAC_ADDRESS_FORMAT = /^([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}$/;
        if (!macAddress || !MAC_ADDRESS_FORMAT.test(macAddress)) {
            return NextResponse.json({ response: "Invalid request data" }, { status: 400 });
        }
    } catch (error) {
        return NextResponse.json({ response: "Internal server error" }, { status: 500 });
    }

    // Updating password
    try {
        const UPDATE_PASSWORD_API = await fetch(`${process.env.BACKEND_API_URL}/api/backend/updateAdminPassword`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                macAddress: macAddress,
                keyBinHash: process.env.KEY_BIN_HASH,
                currentPassword: currentPassword,
                newPassword: newPassword,
                confirmPassword: confirmPassword,
            }),
        });

        if (!UPDATE_PASSWORD_API.ok) {
            const UPDATE_PASSWORD_API_DATA = await UPDATE_PASSWORD_API.json();

            return NextResponse.json({ response: UPDATE_PASSWORD_API_DATA.response }, { status: UPDATE_PASSWORD_API.status });
        }

        return NextResponse.json({ response: "Success" }, { status: 200 });
    } catch (e) {
        return NextResponse.json({ response: "Internal server error" }, { status: 500 });
    }
}
