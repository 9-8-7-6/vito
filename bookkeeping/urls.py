from rest_framework.routers import DefaultRouter
from django.urls import path, include
from .views import AccountViewSet, AssetViewSet, TransactionViewSet

router = DefaultRouter()
router.register('accounts', AccountViewSet, basename='account')
router.register('assets', AssetViewSet, basename='asset')
router.register('transactions', TransactionViewSet, basename='transaction')

urlpatterns = [
    path('', include(router.urls)),  # 這行很重要，確保 REST API 有被註冊！
]
