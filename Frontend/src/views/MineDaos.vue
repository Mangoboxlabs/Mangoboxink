<template>
<div class="MineDaos">
  <div class="content-box">
    <div class="box-nav">
      <div class="item" onClick={()=>{getBalance()}}>
        My Mother Dao
      </div>
      <div class="item">
        My Alliance Dao
      </div>
      <div class="item">
        My Child Dao
      </div>
    </div>
    <div class='Cont'>

      <button>
        点击跳转
      </button>
    </div>
  </div>
</div>
</template>

<script>
export default {
  name: "MineDaos"
}
</script>

<style lang="scss" scoped>
.MineDaos{

}
</style>
