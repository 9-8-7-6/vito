from django.shortcuts import render
from .serializers import AccountSerializer, AssetSerializer, TransactionSerializer
from .models import Account, Asset, Transaction
from rest_framework import viewsets
from rest_framework.response import Response

class AccountViewSet(viewsets.ModelViewSet):
    serializer_class = AccountSerializer
    queryset = Account.objects.all()

class AssetViewSet(viewsets.ModelViewSet):
    serializer_class = AssetSerializer
    queryset = Asset.objects.all()

class TransactionViewSet(viewsets.ModelViewSet):
    serializer_class = TransactionSerializer
    queryset = Transaction.objects.all()

    def create(self, request, *args, **kwargs):
        data = request.data

        # 確保金額大於 0
        if data.get("amount") and float(data["amount"]) <= 0:
            return Response({"error": "交易金額必須大於 0"}, status=status.HTTP_400_BAD_REQUEST)

        serializer = self.get_serializer(data=data)
        serializer.is_valid(raise_exception=True)
        self.perform_create(serializer)
        return Response(serializer.data, status=status.HTTP_201_CREATED)
