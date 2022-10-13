<template>
  <div class="ProjectsView">
    <div class="content-header">
      <div class="nav-list">
        <div class="nav-item" @click="activeIndex=0" :class="{'active':activeIndex==0}">
          Trending
        </div>
        <div class="nav-item" @click="activeIndex=1" :class="{'active':activeIndex==1}">
          My ProjectsView
        </div>
        <div class="nav-item" @click="activeIndex=2" :class="{'active':activeIndex==2}">
          All
        </div>
      </div>
      <div class="right">
        <div class="search-box">
          <input type="text" placeholder="Search"/>
          <svg t="1663312917552" class="icon" viewBox="0 0 1024 1024" version="1.1"
               xmlns="http://www.w3.org/2000/svg" p-id="2342" width="32" height="32">
            <path
                d="M685.6 660.336l155.152 155.168a16 16 0 0 1 0 22.624l-11.312 11.328a16 16 0 0 1-22.624 0l-158.528-158.544a289.792 289.792 0 0 1-165.152 51.36C322.336 742.256 192 611.904 192 451.12 192 290.336 322.336 160 483.136 160c160.784 0 291.12 130.336 291.12 291.136 0 82.112-33.984 156.272-88.672 209.2z m-202.464 33.92c134.272 0 243.12-108.848 243.12-243.12C726.256 316.848 617.408 208 483.136 208 348.848 208 240 316.848 240 451.136c0 134.272 108.848 243.12 243.136 243.12z"
                p-id="2343" fill="#bfbfbf"></path>
          </svg>
        </div>

        <button class="mangobox-button">
          Creat your project
        </button>
      </div>
    </div>
    <div class="ProjectsView-content trending">
      <h2> ProjectsView</h2>
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
        More trending ProjectsView
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "ProjectsView",
  data() {
    return {
      activeIndex: 0
    }
  }
}
</script>

<style lang="scss" scoped>
.ProjectsView {
  overflow: hidden;
  background: #F8F8F8;

  .content-header {
    width: 1200px;
    margin: 40px auto;
    display: flex;
    justify-content: space-between;

    .nav-list {
      display: flex;
      user-select: none;
      .nav-item {
        font-size: 16px;
        font-family: Roboto-Bold, Roboto;
        font-weight: bold;
        color: #333333;
        line-height: 19px;
        margin-right: 30px;
        cursor: pointer;
        position: relative;

        &.active {
          color: #2BAC00;

          &:after {
            bottom: 0;
            left: 0;
            position: absolute;
            content: '';
            width: 100%;
            height: 5px;
            background: linear-gradient(225deg, #54D500 0%, #2AAA00 100%);
          }
        }
      }
    }

    .right {
      display: flex;

      .search-box {
        position: relative;

        input {
          box-sizing: border-box;
          margin-right: 6px;
          padding: 3px 10px;
          height: 40px;
          width: 260px;
          background: #F8F8F8;
          border-radius: 5px;
          border: 1px solid #D8D8D8;
        }

        .icon {
          position: absolute;
          right: 10px;
          top: 5px;
        }
      }

      a {
        text-decoration: none;
      }

      .mangobox-button {
        height: 40px;
        padding: 6px 16px;

      }
    }
  }

  .ProjectsView-content {
    width: 1200px;
    margin: 0 auto;

    .dao-list {
      display: flex;

      .dao-item {

        display: flex;
        background: #fff;
        border-radius: 10px;
        box-shadow: 0px 5px 10px 5px rgba(0, 0, 0, 0.05);
        padding: 10px;
        cursor: pointer;

        .logo {
          width: 70px;
        }

        .right {
          width: 160px;

          .payments, .time {
            margin-top: 6px;
            color: #999;

            span {
              color: #FF7237;
            }
          }
        }
      }
    }

    .more {
      padding: 10px;
      width: 260px;
      margin: 15px auto;
    }
  }
}
</style>
