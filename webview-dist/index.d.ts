import { RequestFlags } from "./bindings/RequestFlags";
export declare function setSearch(query: string): Promise<void>;
export declare function setRequestFlags(flags: RequestFlags[]): Promise<void>;
export declare function setSort(sort: any): Promise<void>;
export declare function setResultOffset(offset: number): Promise<void>;
export declare function setMaxResults(maxResults: number): Promise<void>;
export declare function query(): Promise<void>;
export declare function getNumResults(): Promise<number>;
export declare function getFullPathResults(): Promise<string[]>;
export declare function getResultFullPath(index: number): Promise<string>;
export declare function getFileNameResults(): Promise<string[]>;
export declare function getResultFileName(index: number): Promise<string>;
