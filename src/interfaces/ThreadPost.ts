export interface ThreadPost {
    no: number,
    sticky: number,
    closed: number,
    now: String,
    name: String,
    sub: String,
    com: String,
    filename: String,
    ext: String,
    w: number,
    h: number,
    tn_w: number,
    tn_h: number,
    tim: number,
    time: number,
    md5: String,
    fsize: number,
    resto: number,
    capcode: String,
    semantic_url: String,
    replies: number,
    images: number,
    unique_ips: number
}

export interface ThreadPosts {
    posts: Array<ThreadPost>
}
