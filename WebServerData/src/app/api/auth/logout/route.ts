import { NextRequest, NextResponse } from "next/server";

export async function GET(req: NextRequest) {
    const cookieHeader = req.headers.get("cookie");

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

        const response = NextResponse.json({ response: "Logout successful" }, { status: 200 });

        response.cookies.delete("token");

        return response;
    } catch (error) {
        return NextResponse.json({ response: "Internal server error" }, { status: 500 });
    }
}
