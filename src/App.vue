<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface CopyList {
  id: string
  title: string
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

const copyTitle = ref('')
const textModel = ref('')
const handleAddCopy = () => {
  const newText = textModel.value.trim()

  if (!newText || !copyTitle.value) return

  const payload = {
    id: new Date().getTime().toString(),
    title: copyTitle.value,
    label: newText,
    isEdit: false
  }
  window.localStorage.setItem('copyList', JSON.stringify([...copyList.value, payload]))
  setupCopyList()

  copyTitle.value = ''
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

const onAddClick = () => {
  isAdding.value = true
  setTimeout(() => {
    const title = document.getElementById('add__title')
    title?.focus()
  }, 50);
}

onMounted(() => {
  setupCopyList()
})
</script>

<template>
  <section>
    <ul
      class="w-full overflow-auto backdrop-blur-md space-y-2 px-4 pt-4 h-[calc(100dvh-80px)]"
    >
      <li v-for="item in copyList" :key="item.id" @mouseover="editId = item.id" @mouseleave="editId = ''" class="relative px-4 py-2 border border-black rounded-md">
        <div>
          <p class="line-clamp-1 text-lg fw-700 text-stroke-black">{{ item.title }}</p>
          <p class="text-sm fw-700 text-stone-500 whitespace-pre-line line-clamp-2">{{ item.label }}</p>
        </div>
  
        <button v-show="editId === item.id" @click="handleCopy(item.label)" class="absolute w-1/2 h-full top-0 left-0 bg-#b7ff4a/25 hover:bg-#b7ff4a/50 flex items-center justify-center">
          <img src="./assets/copy.svg" alt="Copy the text" class="w-5 h-5">
        </button>
        <button v-show="editId === item.id" @click="handleDelete(item.id)" class="absolute w-1/2 h-full top-0 left-1/2 bg-red/25 hover:bg-red/50 flex items-center justify-center">
          <img src="./assets/delete.svg" alt="Delete the text" class="w-5 h-5">
        </button>
  
      </li>
      <li v-if="copyList.length === 0 && !isAdding" class="text-black font-bold text-center mt-30 text-lg whitespace-nowrap">
        <p>哦！你還沒有新增快速複製唷</p>
      </li>
  
      <li v-if="isAdding" class="space-y-2 p-2 bg-gray-200 rounded-md">
        <input
          id="add__title"
          type="text"
          v-model="copyTitle"
          placeholder="請輸入標題"
          class="w-full focus:outline-none text-sm text-stone-500 p-2 border border-black rounded-md"
        >
        <textarea
          v-model="textModel"
          placeholder="請輸入文字"
          rows="5"
          class="w-full focus:outline-none text-sm text-stone-500 p-2 border border-black rounded-md"
        />
      </li>
    </ul>
    <div v-if="isAdding" class="fixed bottom-0 w-full p-4 bg-white flex items-center gap-2">
      <button @click="isAdding = false, textModel = ''" class="w-full py-2 bg-white border border-black rounded-md">取消</button>
      <button @click="handleAddCopy" class="w-full py-2 bg-black text-white border border-black rounded-md">新增</button>
    </div>
    <div v-if="!isAdding" class="fixed bottom-0 w-full p-4 bg-white">
      <button @click="onAddClick" class="w-full border border-black py-2 rounded-md bg-white shadow-md hover:bg-black hover:text-white duration-300">新增</button>
    </div>
  </section>
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
  background-color: #b7ff4a;
  border: 1px solid slategrey;
}

*::-webkit-scrollbar-track {
  box-shadow: transparent;
}

.bg-gradient-animation {
  animation: gradient 1s infinite linear;
}

.background-gradient {
	background: linear-gradient(0deg, #4ccaf7, #23d5ab, #23d5ab, #4ccaf7);
	background-size: 400% 400%;
	animation: backgroundgradient 4s infinite linear;
}

@keyframes backgroundgradient {
  0% {
    background-position: 0% 0%;
  }
  100% {
    background-position: 0% 400%;
  }
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
