<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface CopyList {
  id: string
  label: string
  isEdit: boolean
}

const isAdding = ref(false)

const copyList = ref<CopyList[]>([])
const setupCopyList = () => {
  const copyCache = window.localStorage.getItem('copyList')

  if (!copyCache) return

  copyList.value = JSON.parse(copyCache)
  isAdding.value = false
}

const textModel = ref('')
const handleAddText = () => {
  const newText = textModel.value.trim()

  if (!newText) return

  const payload = {
    id: new Date().getTime().toString(),
    label: newText,
    isEdit: false
  }
  window.localStorage.setItem('copyList', JSON.stringify([...copyList.value, payload]))
  setupCopyList()

  textModel.value = ''
}

const editId = ref('')

const handleDelete = (id: string) => {
  const newCopyList = copyList.value.filter(item => item.id !== id)
  window.localStorage.setItem('copyList', JSON.stringify(newCopyList))
  setupCopyList()
}

// 呼叫 Rust 關閉視窗
const handleHideWindow = () => {
  invoke('tauri', { cmd: 'hide_window' })
}

const handleCopy = (text: string) => {
  navigator.clipboard.writeText(text)
  handleHideWindow()
}

onMounted(() => {
  setupCopyList()
})
</script>

<template>
  <ul class="w-full h-screen overflow-auto bg-gradient-to-l from-pink-300 to-green-300 py-4">
    <li v-for="item in copyList" :key="item.id" @mouseover="editId = item.id" @mouseleave="editId = ''" class="relative px-4 py-2">
      <p class="line-clamp-1 text-sm text-#333 fw-700">{{ item.label }}</p>

      <button v-show="editId === item.id" @click="handleCopy(item.label)" class="absolute w-1/2 h-full top-0 left-0 bg-white/25 hover:bg-white/50 backdrop-blur-sm flex items-center justify-center">
        <img src="./assets/copy.svg" alt="Copy the text" class="w-5 h-5">
      </button>
      <button v-show="editId === item.id" @click="handleDelete(item.id)" class="absolute w-1/2 h-full top-0 left-1/2 bg-white/25 hover:bg-white/50 backdrop-blur-sm flex items-center justify-center">
        <img src="./assets/delete.svg" alt="Delete the text" class="w-5 h-5">
      </button>

    </li>
    <li v-show="copyList.length === 0 && !isAdding" class="text-white fw-700 text-center">目前沒有資料唷</li>

    <li v-show="isAdding" class="px-4">
      <div class="rounded-10px overflow-hidden relative w-full h-9">
        <div class="absolute top-0 left-0 w-full h-full border-gradient"></div>
        <input v-model="textModel" @keyup.enter="handleAddText" type="text" class="absolute w-[calc(100%-4px)] left-2px h-[calc(100%-4px)] top-2px rounded-8px px-2 py-1 focus:outline-none text-sm text-stone-500">
      </div>
    </li>

    <li class="flex justify-center mt-2">
      <button v-show="!isAdding" @click="isAdding = true" class="w-1/2 bg-green-400 border border-black py-1">ADD</button>
    </li>
  </ul>
</template>

<style>
*::-webkit-scrollbar {
  width: 7px;
}

*::-webkit-scrollbar-button {
  background: transparent;
  border-radius: 4px;
}

*::-webkit-scrollbar-track-piece {
  background: transparent;
}

*::-webkit-scrollbar-thumb {
  border-radius: 4px;
  background-color: rgb(183, 255, 74);
  border: 1px solid slategrey;
}

*::-webkit-scrollbar-track {
  box-shadow: transparent;
}

.border-gradient {
	background: linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab);
	background-size: 400% 400%;
	animation: gradient 1s ease infinite;
}

@keyframes gradient {
	0% {
		background-position: 0% 50%;
	}
	50% {
		background-position: 100% 50%;
	}
	100% {
		background-position: 0% 50%;
	}
}
</style>
