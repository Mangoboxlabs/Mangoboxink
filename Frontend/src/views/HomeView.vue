<template>
  <div class="home">
    <div class="home-content">
      <h2>Trending projects</h2>
      <div class="dao-list">

        <div class="dao-item" @click="$router.push({path:'/Details'})">
          <div class="logo">
            <img src="" alt=""/>
          </div>
          <div class="right">
            <div class="name">
              MangoBox
            </div>
            <div class="time">
              Last 7days <span>+10%</span>
            </div>
            <div class="payments">
              28payments
            </div>
          </div>
        </div>
      </div>

      <div class="more mangobox-button">
        More trending projects
      </div>
  </div>
  </div>
</template>

<script>


export default {
  name: 'HomeView',
  components: {

  }
}
</script>
<style lang="scss">
.home{
  background: #F8F8F8;
  overflow: hidden;
  .home-content {
    width: 1200px;
    margin: 20px auto;
    .dao-list {
      display: flex;

      .dao-item {
        display: flex;
        background: #fff;
        border-radius: 10px;
        box-shadow: 0px 5px 10px 5px rgba(0,0,0,0.05);
        padding: 10px;
        cursor: pointer;
        .logo {
          width: 70px;
        }
        .right{
          width: 160px;
          .payments,.time{
            margin-top: 6px;
            color: #999;
            span{
              color: #FF7237;
            }
          }
        }
      }
    }
    .more{
      padding: 10px;
      width: 260px;
      margin: 15px auto;
    }
  }
}
</style>
