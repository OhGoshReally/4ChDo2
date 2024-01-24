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

      <div v-if="threadsAreLoading" class="thread-spinner-big">
        <div class="spinner-border" role="status">
          <span class="visually-hidden">Loading...</span>
        </div>
      </div>

      <div v-if="showBoardThreads" class="thread-selector-container col d-flex flex-row justify-content-center my-4 px-4">
        <div :class="threadBackClass" @click="stepThreadsBackOrForward(false)">
          <font-awesome-icon :icon="['fas', 'caret-left']" size="2xl"/>
        </div>
        <div class="thread-selector">
            <Thumbnail
                v-for="(thread, index) in groupedCatalogThreads[selectedCatalogThreadIndex]"
                @click-thread="selectThread(thread.no)"
                :thread="thread"
                :board="selectedBoard"
                :index="index"
                :selectedThread="selectedThread"
                :key="thread.no"
            ></Thumbnail>
        </div>
        <div :class="threadForwardClass" @click="stepThreadsBackOrForward(true)">
          <font-awesome-icon :icon="['fas', 'caret-right']" size="2xl"/>
        </div>
      </div>

      <div class="col-5" v-if="boardIsSelected">
        <div class="mb-3">
          <label for="thread_id_input" class="form-label fw-bold">Thread ID</label>
          <div :class="selectedThreadGroupStyling">
            <span class="input-group-text input-group-text-start">#</span>
            <input v-model="selectedThread" id="thread_id_input" class="form-control" :disabled="threadsAreLoading" placeholder="ex. 570368" type="text" spellcheck="false">
            <div class="input-group-text input-group-text-end">
              <div v-if="fetchingThread" class="spinner-border" role="status">
                <span class="visually-hidden">Loading...</span>
              </div>
            </div>
          </div>
          <div v-if="showThreadData" :class="threadDataClass">
            <p class="m-0 fs-6">{{ imagesInThread }} images</p>
            <p class="m-0 fs-6">Total size: {{ totalFileSizeFormatted }}</p>
          </div>
        </div>
        <button @click="getThreadData" class="btn btn-primary w-100">Click here!!!</button>
      </div>
    </div>
  </div>

  <p class="mt-3">{{ greetMsg }}</p>
  <p class="mt-3">{{ files }}</p>
</template>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Boards, BoardObject } from "../interfaces/Boards";
  import { CatalogPage, CatalogThread } from "../interfaces/Catalog";
  import { ThreadPost, ThreadPosts } from "../interfaces/ThreadPost";
  import Thumbnail from "./ThumbnailComponent.vue";
  export default {
    name: "Main",
    components: { Thumbnail },
    data() : {
      name: string,
      greetMsg: string,
      files: string,
      boardsIsLoading: boolean,
      threadsAreLoading: boolean,
      availableBoards: Array<BoardObject>,
      selectedBoard: string,
      selectedThread: string,
      selectedCatalogThreads: Array<CatalogThread>,
      selectedCatalogThreadIndex: number,
      fetchingThread: boolean,
      threadsPerThreadGroup: number,
      threadPosts: Array<ThreadPost>
    } {
      return {
        name: '',
        greetMsg: '',
        files: '',
        boardsIsLoading: false,
        threadsAreLoading: false,
        availableBoards: [],
        selectedBoard: 'none',
        selectedThread: '',
        selectedCatalogThreads: [],
        selectedCatalogThreadIndex: 0,
        fetchingThread: false,
        threadsPerThreadGroup: 6,
        threadPosts: []
      };
    },
    computed: {
      showThreadData(): boolean {
        return this.threadPosts.length > 0
      },
      threadDataClass(): object {
        return {
          'opacity-50': this.fetchingThread
        }
      },
      threadBackClass(): object {
        return {
          'thread-arrow-left': true,
          'clickable': this.canStepThreadsBack,
          'opacity-50': !this.canStepThreadsBack
        }
      },
      threadForwardClass(): object {
        return {
          'thread-arrow-right': true,
          'clickable': this.canStepThreadsForward,
          'opacity-50': !this.canStepThreadsForward
        }
      },
      canStepThreadsBack(): boolean {
        return this.selectedCatalogThreadIndex !== 0;
      },
      canStepThreadsForward(): boolean {
        return this.selectedCatalogThreadIndex !== this.groupedCatalogThreads.length - 1;
      },
      groupedCatalogThreads(): Array<Array<CatalogThread>> {
        return this.groupThreads(this.selectedCatalogThreads);
      },
      availableThreadsAsStrings(): Array<string> {
        return this.selectedCatalogThreads.map(b => b.no.toString());
      },
      selectedThreadGroupStyling(): object {
        return {
          'input-group': true,
          'input-group-pretext': true,
          'disabled': this.threadsAreLoading,
          'mb-2': true,
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
        return !this.threadsAreLoading && this.selectedCatalogThreads.length > 0;
      },
      imagesInThread(): number {
        return this.threadPosts.filter(p => p.filename).length;
      },
      totalFileSizeInBytes(): number {
        return this.threadPosts.filter(p => p.filename).map(p => p.fsize).reduce((a, b) => a + b, 0);
      },
      totalFileSizeFormatted(): string {
        return this.formatBytes(this.totalFileSizeInBytes);
      }
    },
    methods: {
      formatBytes(bytes: number, decimals = 2): string {
        if (bytes === 0) return '0 Bytes';

        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

        const i = Math.floor(Math.log(bytes) / Math.log(k));

        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
      },
      stepThreadsBackOrForward(increment: boolean): void {
        if (increment && this.canStepThreadsForward) {
          this.selectedCatalogThreadIndex++;
        }

        if (!increment && this.canStepThreadsBack) {
          this.selectedCatalogThreadIndex--;
        }
      },
      groupThreads(threadList: Array<CatalogThread>): Array<Array<CatalogThread>> {
        console.log(threadList)
        if (threadList === undefined || threadList.length == 0) {
          return [ threadList ];
        }

        let groupedList: Array<Array<CatalogThread>> = [];
        let localList: Array<CatalogThread> = [];

        for (let i = 0; i < threadList.length; i++) {
          if (localList.length >= this.threadsPerThreadGroup) {
            groupedList.push(localList);
            localList = [];
          }

          localList.push(threadList[i]);
        }

        if (localList.length > 0) {
          groupedList.push(localList);
          localList = [];
        }

        return groupedList;
      },
      async getAllBoards(): Promise<void> {
        this.boardsIsLoading = true;
        const boards: Boards = await invoke("fetch_boards");
        this.availableBoards = boards.boards;
        console.log(boards);
        this.boardsIsLoading = false;
      },
      async getCatalogForSelectedBoard(): Promise<void> {
        this.threadsAreLoading = true;

        if (this.selectedBoard !== 'none') {
          const catalogPages: Array<CatalogPage> = await invoke("fetch_catalog", { board: this.selectedBoard });
          console.log(catalogPages);
          this.selectedCatalogThreads = this.flattenCatalogThreadStructure(catalogPages);
        } else {
          this.selectedCatalogThreads = [];
        }

        this.threadsAreLoading = false;
      },
      flattenCatalogThreadStructure(pages: Array<CatalogPage>): Array<CatalogThread> {
        return pages.map(p => p.threads).flat().sort((a, b) => (a.time > b.time) ? 1 : -1);
      },
      selectThread(threadNo: number): void {
        this.selectedThread = threadNo.toString();
        this.getThreadData();
      },
      async getThreadData(): Promise<void> {
        if (this.fetchingThread) return;

        this.fetchingThread = true;

        const threadData: ThreadPosts = await invoke("fetch_thread", {
          board: this.selectedBoard,
          thread: this.selectedThread
        });

        this.threadPosts = threadData.posts;

        this.fetchingThread = false;
      }
    },
    mounted(){
      this.getAllBoards();
    }
  };
</script>
