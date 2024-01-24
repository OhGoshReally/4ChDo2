<template>
  <div class="thumbnail-image-container">
    <div :class="thumbnailImageClass" :style="thumbnailStyling" @click="clickThread">
      <video v-if="isWebm" class="video-player" controls width="120">
        <source :src="imageUrl" type="video/webm">
      </video>
      <div v-if="!hasLoaded" class="thumbnail-placeholder">
        <font-awesome-icon :icon="['fas', 'image']" size="lg" class="m-auto"/>
      </div>
      <div v-if="hasLoaded" class="thumbnail-image-overlay"></div>
    </div>
    <div :class="thumbnailPopoverContainerStyling">
      <div class="thumbnail-popover" :style="`background-image: url(${ imageUrl })`">
        <div class="thumbnail-popover-overlay"></div>
        <h5 class="mb-3 fw-bold z-0"># {{ thread.no }}</h5>
        <h5 class="popover-title-text z-0">{{ thread.sub }}</h5>
        <p class="popover-body-text z-0" v-html="thread.com"></p>
        <p class="d-flex flex-row align-items-center m-auto z-0">
          <font-awesome-icon :icon="['fas', 'image']" size="xs" class="me-2"/>
          <span class="fw-semibold">{{ thread.images }} images</span>
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { PropType, StyleValue } from 'vue'
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { CatalogThread } from "../interfaces/Catalog"

export default {
    name: 'Thumbnail',
    props: {
      thread: {
        type: Object as PropType<CatalogThread>,
        required: true
      },
      board: {
        type: String,
        required: true
      },
      index: {
        type: Number,
        required: true
      },
      selectedThread: {
        type: String,
        required: true
      }
    },
    data() : {
      imageUrl: string,
      isWebm: boolean,
      hasLoaded: boolean
    } {
      return {
        imageUrl: '',
        isWebm: false,
        hasLoaded: false
      };
    },
    computed: {
      thumbnailImageClass(): object {
        return {
          'thumbnail-image': true,
          'active': this.selectedThread == this.thread.no.toString()
        }
      },
      thumbnailPopoverContainerStyling(): object {
        return {
          'thumbnail-popover-container': true,
          'left-align': this.index > 0,
          'right-align': this.index == 0
        }
      },
      thumbnailStyling(): StyleValue {
        return {
          backgroundImage: this.hasLoaded ? `url(${ this.imageUrl })` : ''
        }
      }
    },
    methods: {
      async setThumbnailPath(): Promise<void> {
        const args = {
          board: this.board,
          fileName: `${ this.thread.tim }${ this.thread.ext }`
        }
        const imageUrl: string = await invoke("fetch_thumbnail_from_thread", args);
        this.imageUrl = convertFileSrc(imageUrl);
        this.isWebm = this.thread.ext === ".webm";
        this.hasLoaded = true;
      },
      clickThread(){
        this.$emit('clickThread')
      }
    },
    async mounted(){
      await this.setThumbnailPath();
    }
  };
</script>
