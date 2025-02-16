from rest_framework.routers import DefaultRouter
from django.urls import path, include
from .views import AccountViewSet, AssetViewSet, TransactionViewSet, UserViewSet

router = DefaultRouter()
router.register('accounts', AccountViewSet, basename='account')
router.register('assets', AssetViewSet, basename='asset')
router.register('transactions', TransactionViewSet, basename='transaction')
router.register('user', UserViewSet, basename='user')

urlpatterns = [
    path('', include(router.urls)),
]
