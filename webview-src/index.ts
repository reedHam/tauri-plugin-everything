import { invoke } from "@tauri-apps/api/tauri";
import { RequestFlags } from "./bindings/RequestFlags";

export async function setSearch(query: string): Promise<void> {
    await invoke("plugin:everything|set_search", { query });
}

export async function setRequestFlags(flags: RequestFlags[]): Promise<void> {
    await invoke("plugin:everything|set_request_flags", { flags });
}

export async function setSort(sort: any): Promise<void> {
    await invoke("plugin:everything|set_sort", { sort });
}

export async function setResultOffset(offset: number): Promise<void> {
    await invoke("plugin:everything|set_result_offset", { offset });
}

export async function setMaxResults(maxResults: number): Promise<void> {
    await invoke("plugin:everything|set_max_results", { maxResults });
}

export async function query(): Promise<void> {
    await invoke("plugin:everything|query");
}

export async function getNumResults(): Promise<number> {
    const numResults = await invoke<number>(
        "plugin:everything|get_num_results"
    );
    return numResults;
}

export async function getFullPathResults(): Promise<string[]> {
    const results = await invoke<string[]>(
        "plugin:everything|get_full_path_results"
    );
    return results;
}

export async function getResultFullPath(index: number): Promise<string> {
    const fullPath = await invoke<string>(
        "plugin:everything|get_result_full_path",
        { index }
    );
    return fullPath;
}

export async function getFileNameResults(): Promise<string[]> {
    const results = await invoke<string[]>(
        "plugin:everything|get_file_name_results"
    );
    return results;
}

export async function getResultFileName(index: number): Promise<string> {
    const fileName = await invoke<string>(
        "plugin:everything|get_result_file_name",
        { index }
    );
    return fileName;
}
