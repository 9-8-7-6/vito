from rest_framework.routers import DefaultRouter
from .views import AccountViewSet, AssetViewSet, TransactionViewSet

router = DefaultRouter()
router.register('accounts', AccountViewSet, basename='user的餘額')
router.register('assets', AssetViewSet, basename='user的該資產的餘額')
router.register('transactions', TransactionViewSet, basename='交易紀錄')


urlpatterns = router.urls