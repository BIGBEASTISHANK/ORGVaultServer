import { NextRequest } from "next/server";

export async function POST(req: NextRequest) {
    const REQUEST_DATA = await req.json();
}
