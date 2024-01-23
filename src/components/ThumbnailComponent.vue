<template>
  <div class="thumbnail-image-container">
    <div class="thumbnail-image" :style="thumbnailStyling" @click="clickThread">
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

<style scoped>
  .thumbnail-image-container,
  .thumbnail-image {
    height: 100px;
    width: 120px;
    position: relative;
  }

  .thumbnail-image {
    background-color: white;
    border-radius: 1rem;
    border-width: 2px;
    border-style: solid;
    border-color: transparent;
    cursor: pointer;
    background-repeat: no-repeat;
    background-position: center;
    background-size: cover;
    background-clip: content-box;
    overflow: hidden;
    display: flex;
  }

  .thumbnail-image,
  .thumbnail-image-overlay {
    transition: border-color .10s ease-in-out, box-shadow .10s ease-in-out, opacity .10s ease-in-out;
  }

  .thumbnail-image:hover {
    border-color: #ffccaa;
  }

  .thumbnail-image:hover .thumbnail-image-overlay {
    opacity: 0.25;
  }

  .thumbnail-image:active .thumbnail-image-overlay {
    opacity: 0.5;
  }

  .thumbnail-image:active {
    box-shadow: 0 0 7px 0 #ffccaa;
  }

  .video-player {
    width: 100%;
    height: 100%;
    margin-bottom: auto;
  }

  .thumbnail-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    background-color: rgba(255, 204, 170, 0.75);
    opacity: 0.75;
  }

  .thumbnail-image-overlay {
    height: 100%;
    width: 100%;
    background-color: #ffffee;
    opacity: 0;
  }

  .thumbnail-popover-container {
    position: absolute;
    top: 0;
  }

  .thumbnail-popover-container.left-align {
    left: -185px;
  }

  .thumbnail-popover-container.right-align {
    right: -5px;
  }

  .thumbnail-popover {
    width: 180px;
    /*background-color: #ffffee;*/
    background-color: rgba(255, 255, 238, 0.8);
    position: fixed;
    z-index: 50;
    border-radius: 1rem;
    border-width: 2px;
    border-style: solid;
    /*border-color: #ffccaa;*/
    border-color: rgba(255, 204, 170, 0.8);
    box-shadow: 0 7px 6px 0 #00000024, 0 2px 3px 0 #00000024;
    display: flex;
    flex-direction: column;
    text-align: center;
    padding: 1.25rem .5rem 1.25rem .5rem;
    background-repeat: no-repeat;
    background-position: center;
    background-size: cover;
    background-clip: border-box;
  }

  .thumbnail-popover-overlay {
    height: 100%;
    width: 100%;
    background-color: rgb(255 255 238 / 85%);
    position: absolute;
    top: 0;
    left: 0;
    border-radius: 1rem;
    box-shadow: 0 0 0 3px #ffccaa;
  }

  .thumbnail-image:not(:hover) + .thumbnail-popover-container {
    display: none;
  }

  .thumbnail-image:hover + .thumbnail-popover-container {
    display: block;
  }

  .popover-title-text {
    font-size: 14px;
    font-weight: 700;
    margin: 0 0 6px 0;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .popover-body-text {
    font-size: 11px;
    line-height: normal;
    margin-bottom: 1rem;
    font-weight: 600;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
  }
</style>
