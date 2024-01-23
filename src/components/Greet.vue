<template>
  <div>
    <div class="row justify-content-center flex-column align-items-center">
      <div class="col-5">
        <label for="exampleFormControlInput1" class="form-label fw-bold mb-2">Board</label>
        <div class="position-relative">
          <select v-model="selectedBoard" class="form-select" aria-label="Default select example" v-on:change="getCatalogForSelectedBoard" :disabled="boardSelectorDisabled">
            <option selected value="none">{{ boardSelectorPlaceholderText }}</option>
            <option v-for="board in availableBoards" :value="board.board">{{ board.title }}</option>
          </select>
          <div v-if="boardsIsLoading" class="spinner-border board-spinner" role="status">
            <span class="visually-hidden">Loading...</span>
          </div>
        </div>
      </div>

      <div v-if="showBoardThreads" class="col d-flex flex-row justify-content-center my-4 px-5">
        <div class="d-flex flex-column my-auto mx-2">
          <font-awesome-icon :icon="['fas', 'caret-left']" size="2xl"/>
        </div>
        <div class="d-flex flex-row flex-shrink-1 overflow-hidden gap-3 flex-grow-1 flex-nowrap justify-content-between">
            <Thumbnail v-for="(thread, index) in selectedCatalogThreads" @click-thread="selectThread(thread.no)" :thread="thread" :board="selectedBoard" :index="index" :key="thread.no"></Thumbnail>
        </div>
        <div class="d-flex flex-column my-auto mx-2">
          <font-awesome-icon :icon="['fas', 'caret-right']" size="2xl"/>
        </div>
      </div>

      <div class="col-5" v-if="boardIsSelected">
        <div class="my-3">
          <label for="thread_id_input" class="form-label fw-bold">Thread ID</label>
          <div :class="selectedThreadGroupStyling">
            <span class="input-group-text" id="basic-addon1">#</span>
            <input v-model="selectedThread" id="thread_id_input" class="form-control" placeholder="ex. 570368" type="text" spellcheck="false">
          </div>
        </div>
        <button @click="testConvertFileSrc" class="btn btn-primary w-100 mt-3">Click here!!!</button>
      </div>
    </div>
  </div>

  <p class="mt-3">{{ greetMsg }}</p>
  <p class="mt-3">{{ files }}</p>
</template>

<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
  import { appDataDir, join } from '@tauri-apps/api/path';
  import { Boards, BoardObject } from "../interfaces/Boards"
  import { CatalogPage, CatalogThread } from "../interfaces/Catalog"
  import Thumbnail from "./ThumbnailComponent.vue";
  export default {
    name: "Greet",
    components: { Thumbnail },
    data() : {
      name: string,
      greetMsg: string,
      files: string,
      boardsIsLoading: Boolean,
      availableBoards: Array<BoardObject>,
      selectedBoard: string,
      selectedThread: string,
      selectedCatalogThreads: Array<CatalogThread>
    } {
      return {
        name: '',
        greetMsg: '',
        files: '',
        boardsIsLoading: false,
        availableBoards: [],
        selectedBoard: 'none',
        selectedThread: '',
        selectedCatalogThreads: []
      };
    },
    computed: {
      availableThreadsAsStrings(): Array<string> {
        return this.selectedCatalogThreads.map(b => b.no.toString());
      },
      selectedThreadGroupStyling(): object {
        return {
          'input-group': true,
          'input-group-pretext': true,
          'mb-3': true,
          'valid': this.selectedThread.length > 0 && this.availableThreadsAsStrings.includes(this.selectedThread),
          'invalid': this.selectedThread.length > 0 && !this.availableThreadsAsStrings.includes(this.selectedThread)
        }
      },
      boardSelectorDisabled(): boolean {
        return this.availableBoards.length === 0;
      },
      boardSelectorPlaceholderText(): String {
        return this.boardSelectorDisabled ? "-- No boards available --" : "-- Select board --";
      },
      boardIsSelected(): boolean {
        return this.selectedBoard !== 'none';
      },
      showBoardThreads(): boolean {
        return this.selectedCatalogThreads.length > 0;
      }
    },
    methods: {
      async getAllBoards() {
        this.boardsIsLoading = true;
        const boards: Boards = await invoke("fetch_boards");
        this.availableBoards = boards.boards;
        console.log(boards);
        this.boardsIsLoading = false;
      },
      async getCatalogForSelectedBoard() {
        this.boardsIsLoading = true;

        if (this.selectedBoard !== 'none') {
          const catalogPages: Array<CatalogPage> = await invoke("fetch_catalog", { board: this.selectedBoard });
          console.log(catalogPages);
          this.selectedCatalogThreads = this.flattenCatalogThreadStructure(catalogPages);
        } else {
          this.selectedCatalogThreads = [];
        }

        this.boardsIsLoading = false;
      },
      flattenCatalogThreadStructure(pages: Array<CatalogPage>): Array<CatalogThread> {
        return pages.map(p => p.threads).flat().sort((a, b) => (a.time > b.time) ? 1 : -1);
      },
      async testConvertFileSrc(fileName: string): Promise<string> {
        const appDataDirPath = await appDataDir();
        console.log(appDataDirPath)
        const filePath = await join(await join(appDataDirPath, 'thumbnails'), fileName);
        const assetUrl = convertFileSrc(filePath);
        console.log(assetUrl)
        return assetUrl;
      },
      selectThread(threadNo: number){
        this.selectedThread = threadNo.toString();
      }
    },
    mounted(){
      this.getAllBoards();
    }
  };
</script>
