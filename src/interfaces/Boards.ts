export interface Cooldowns {
    threads: number;
    replies: number;
    images: number;
}
export interface BoardObject {
    board: String;
    title: String;
    ws_board: number;
    per_page: number;
    pages: number;
    max_filesize: number;
    max_webm_filesize: number;
    max_comment_chars: number;
    max_webm_duration: number;
    bump_limit: number;
    image_limit: number;
    cooldowns: Cooldowns;
    meta_description: String;
    is_archived: number
}
export interface Boards {
    boards: Array<BoardObject>
}
